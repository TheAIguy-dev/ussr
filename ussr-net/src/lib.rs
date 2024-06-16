use std::{
    io::{self, Cursor, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    ops::RangeInclusive,
    time::Instant,
};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bytes::{Buf, BytesMut};
use tracing::{info, instrument, trace, warn};
use ussr_buf::{ReadError, VarReadable, VarSize, VarWritable};
use ussr_protocol::{
    proto::{
        enums::State,
        packets::{handshaking, login, status},
    },
    Packet, PacketReadError,
};

pub struct UssrNetPlugin;
impl Plugin for UssrNetPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.insert_resource(Server::new()).add_systems(
            Update,
            (
                accept_connections,
                read_data,
                check_frames,
                // packet handlers
                (
                    handle_handshake,
                    handle_status_request,
                    handle_ping_request,
                    handle_login_start,
                ),
            )
                .chain(),
        );
    }
}

/// The size of the read buffer.
const READ_BUFFER_SIZE: usize = 1024;

/// A resource that contains server information.
/// The underlying [`TcpListener`] must be non-blocking, see [`TcpListener::set_nonblocking`].
#[derive(Resource)]
struct Server {
    listener: TcpListener,
}
impl Server {
    fn new() -> Self {
        let listener: TcpListener =
            TcpListener::bind("127.0.0.1:25565").expect("Failed to bind server on port 25565");
        listener
            .set_nonblocking(true)
            .expect("Failed to set server to non-blocking");
        Self { listener }
    }
}

/// A single connection to the server.
/// This component is added automatically by [`accept_connections`].
#[derive(Component)]
pub struct Connection {
    stream: TcpStream,
    state: State,
    incoming_buf: BytesMut, //? Maybe it should be a vector of frames
}
impl Connection {
    /// Create a new connection.
    /// The stream will be made non-blocking, see [`TcpStream::set_nonblocking`].
    fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self {
            stream,
            state: State::Handshaking,
            incoming_buf: BytesMut::new(),
        })
    }
}

#[derive(Component)]
struct StartTime(Instant);

/// A system that accepts connections and spawns new entities with [`Connection`].
/// The listener must be non-blocking, see [`TcpListener::set_nonblocking`].
#[instrument(skip_all, level = "trace")]
fn accept_connections(mut commands: Commands, server: Res<Server>) {
    loop {
        if let Ok((stream, _)) = server.listener.accept() {
            trace!("Accepted connection");
            if let Ok(connection) = Connection::new(stream) {
                commands.spawn(connection);
            } else {
                return;
            }
        } else {
            return;
        }
    }
}

/// A system that reads data from a [`Connection`].
/// The connection must be non-blocking, see [`TcpStream::set_nonblocking`].
#[instrument(skip_all, level = "trace")]
fn read_data(mut commands: Commands, mut query: Query<(Entity, &mut Connection)>) {
    let mut buf: Vec<u8> = vec![0; READ_BUFFER_SIZE];

    for (entity, mut connection) in &mut query {
        match connection.stream.read(&mut buf) {
            // Connection closed
            Ok(0) => {
                trace!("Connection closed");
                commands.entity(entity).despawn();
                // commands.entity(entity).remove::<Connection>();
            }
            // Successful read
            Ok(n) => {
                trace!("Read {n} bytes");
                connection.incoming_buf.extend_from_slice(&buf[..n]);
                if connection.incoming_buf.len() == 16000000 {
                    info!("Got 1_000_000 handshakes, starting timer");
                    commands.entity(entity).insert(StartTime(Instant::now()));
                } else if connection.incoming_buf.len() == 16000 {
                    info!("Got 1_000 handshakes, starting timer");
                    commands.entity(entity).insert(StartTime(Instant::now()));
                }
            }
            // Wood block
            Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
            // Other error
            Err(e) => {
                trace!("Error: {e:?}");
                continue;
            }
        }
    }
}

#[instrument(skip_all, level = "trace")]
fn check_frames(mut commands: Commands, mut query: Query<(Entity, &mut Connection, &StartTime)>) {
    'entities: for (entity, mut connection, start_time) in &mut query {
        while !connection.incoming_buf.is_empty() {
            let mut buf: Cursor<&[u8]> = Cursor::new(&connection.incoming_buf[..]);
            let og_len: usize = buf.remaining();
            // Try to read the packet length
            match usize::read_var_from(&mut buf) {
                Ok(packet_length) => {
                    let len_before_id: usize = buf.remaining();
                    // Check if we have enough bytes to parse the packet
                    if packet_length <= buf.remaining() {
                        if packet_length < usize::MIN_SIZE {
                            trace!("Invalid packet length (0), despawning entity");
                            commands.entity(entity).despawn();
                            continue 'entities;
                        }

                        // Parse the packet
                        match parse_packet(
                            &mut commands,
                            entity,
                            packet_length,
                            connection.state,
                            &mut buf,
                        ) {
                            Ok(is_valid_len) => {
                                let new_len: usize = buf.remaining();
                                // Ensure that we read the correct amount of bytes
                                if is_valid_len {
                                    connection.incoming_buf.advance(og_len - new_len);
                                } else {
                                    trace!("Invalid packet length ({packet_length}, read {} bytes), despawning entity", len_before_id - new_len);
                                    commands.entity(entity).despawn();
                                }
                            }
                            Err(PacketReadError::Io(_)) => continue 'entities,
                            Err(e) => {
                                trace!("Parse error: {e:?}, despawning entity");
                                commands.entity(entity).despawn();
                                continue 'entities;
                            }
                        }
                    }
                }
                Err(ReadError::InvalidVarInt) => {
                    trace!("Invalid packet length, despawning entity");
                    commands.entity(entity).despawn();
                    continue 'entities;
                }
                // Every other error is an IO error.
                // This means that we don't have enough bytes to read the packet length.
                Err(_) => continue 'entities,
            }
        }

        if connection.incoming_buf.is_empty() {
            info!("Empty buffer, took: {:?}", start_time.0.elapsed());
            commands.entity(entity).remove::<StartTime>();
            continue;
        }
    }
}

/// The current implementation will parse the packet id, then the packet itself. It will insert that packet as a component to the given entity.
///
/// It will return whether the packet length was valid, AKA contained in the allowed range for the packet.
#[instrument(skip_all, level = "trace")]
fn parse_packet(
    commands: &mut Commands,
    entity: Entity,
    packet_length: usize,
    state: State,
    buf: &mut Cursor<&[u8]>,
) -> Result<bool, PacketReadError> {
    let len_before_id: usize = buf.remaining();
    let packet_id: u32 = u32::read_var_from(buf)?;
    let packet_id_len: usize = len_before_id - buf.remaining();

    let range: RangeInclusive<usize> = match state {
        State::Handshaking => match packet_id {
            handshaking::serverbound::Handshake::ID => {
                trace!("Reading handshake");
                let p = handshaking::serverbound::Handshake::read(buf)?;
                commands.entity(entity).insert(p);
                handshaking::serverbound::Handshake::MIN_SIZE
                    ..=handshaking::serverbound::Handshake::MAX_SIZE
            }
            _ => return Err(PacketReadError::UnknownPacketId { packet_id, state }),
        },
        State::Status => match packet_id {
            status::serverbound::StatusRequest::ID => {
                trace!("Reading status request");
                let p = status::serverbound::StatusRequest::read(buf)?;
                commands.entity(entity).insert(p);
                status::serverbound::StatusRequest::MIN_SIZE
                    ..=status::serverbound::StatusRequest::MAX_SIZE
            }
            status::serverbound::PingRequest::ID => {
                trace!("Reading ping request");
                let p = status::serverbound::PingRequest::read(buf)?;
                commands.entity(entity).insert(p);
                status::serverbound::PingRequest::MIN_SIZE
                    ..=status::serverbound::PingRequest::MAX_SIZE
            }
            _ => return Err(PacketReadError::UnknownPacketId { packet_id, state }),
        },
        State::Login => match packet_id {
            login::serverbound::LoginStart::ID => {
                trace!("Reading login start");
                let p = login::serverbound::LoginStart::read(buf)?;
                commands.entity(entity).insert(p);
                login::serverbound::LoginStart::MIN_SIZE..=login::serverbound::LoginStart::MAX_SIZE
            }
            _ => return Err(PacketReadError::UnknownPacketId { packet_id, state }),
        },
        State::Play => unimplemented!(),
    };

    Ok(range.contains(&(packet_length - packet_id_len)))
}

#[instrument(skip_all, level = "trace")]
fn handle_handshake(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Connection,
            &handshaking::serverbound::Handshake,
        ),
        Added<handshaking::serverbound::Handshake>,
    >,
) {
    for (entity, mut connection, handshake) in &mut query {
        trace!("Handshake received, next state: {}", handshake.next_state);
        // connection.state = handshake.next_state.into();
        commands
            .entity(entity)
            .remove::<handshaking::serverbound::Handshake>();
    }
}

#[instrument(skip_all, level = "trace")]
fn handle_status_request(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Connection), Added<status::serverbound::StatusRequest>>,
) {
    for (entity, mut connection) in &mut query {
        trace!("Status request received");
        let buf: Vec<u8> = serialize_packet(status::clientbound::StatusResponse {
            response: r#"
            {
                "version": {
                    "name": "1.7.2",
                    "protocol": 4
                },
                "players": {
                    "max": 100,
                    "online": 0,
                    "sample": []
                },
                "description": {
                    "text": "Hello, world!"
                }
            }
            "#
            .to_string(),
        });
        connection.stream.write_all(&buf).unwrap();
        commands
            .entity(entity)
            .remove::<status::serverbound::StatusRequest>();
    }
}

#[instrument(skip_all, level = "trace")]
fn handle_ping_request(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Connection, &status::serverbound::PingRequest),
        Added<status::serverbound::PingRequest>,
    >,
) {
    for (entity, mut connection, ping_request) in &mut query {
        trace!("Ping request received");
        let buf: Vec<u8> = serialize_packet(status::clientbound::PingResponse {
            payload: ping_request.payload,
        });
        connection.stream.write_all(&buf).unwrap();
        commands.entity(entity).despawn();
    }
}

#[instrument(skip_all, level = "trace")]
fn handle_login_start(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Connection, &login::serverbound::LoginStart),
        Added<login::serverbound::LoginStart>,
    >,
) {
    for (entity, mut connection, login_start) in &mut query {
        trace!("Login start received");
        let buf: Vec<u8> = serialize_packet(login::clientbound::Disconnect {
            reason: format!(
                r#"
            {{
                "text": "Fuck off, {}",
                "color": "red",
                "bold": true
            }}
            "#,
                login_start.username
            ),
        });
        connection.stream.write_all(&buf).unwrap();
        commands.entity(entity).despawn();
    }
}

#[instrument(skip_all, level = "trace")]
pub fn serialize_packet<T: Packet>(packet: T) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    T::ID.write_var_to(&mut buf).unwrap();
    packet.write(&mut buf).unwrap();
    let len: usize = buf.len();
    len.write_var_to(&mut buf).unwrap();
    let new_len: usize = buf.len();
    buf.rotate_right(new_len - len);
    buf
}

// TODO: system for writing outgoing data
// TODO: function to prefix the packet with its length
