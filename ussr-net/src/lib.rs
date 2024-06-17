use std::{
    io::{self, Cursor, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    time::Instant,
};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bytes::{Buf, BytesMut};
use tracing::{info, instrument, trace, warn};
use ussr_buf::{ReadError, VarReadable, VarSize, VarWritable};
use ussr_net_macros::packet_decoder_map;
use ussr_protocol::{
    proto::{
        enums::State,
        packets::{
            handshaking::serverbound::*,
            login::{clientbound::*, serverbound::*},
            status::{clientbound::*, serverbound::*},
        },
    },
    Packet, PacketReadError,
};

pub struct UssrNetPlugin;
impl Plugin for UssrNetPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        #[cfg(feature = "bench")]
        info!("Starting with benchmark mode active");

        app.add_event::<PacketEvent<Handshake>>();
        app.add_event::<PacketEvent<StatusRequest>>();
        app.add_event::<PacketEvent<PingRequest>>();
        app.add_event::<PacketEvent<LoginStart>>();

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
// TODO: Add timeout
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
struct JoinTime(Instant);

#[derive(Event)]
struct PacketEvent<T: Packet> {
    entity: Entity,
    packet: T,
}

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
                #[cfg(not(feature = "bench"))]
                {
                    trace!("Connection closed");
                    //? Maybe we shouldn't despawn the entity, and process the leftower data?
                    commands.entity(entity).despawn();
                }
                // commands.entity(entity).remove::<Connection>();
            }
            // Successful read
            Ok(n) => {
                trace!("Read {n} bytes");
                connection.incoming_buf.extend_from_slice(&buf[..n]);
                #[cfg(feature = "bench")]
                {
                    if connection.incoming_buf.len() == 20000016 {
                        info!("Got 10_000_000 status requests, starting timer");
                        commands.entity(entity).insert(JoinTime(Instant::now()));
                    }
                }
                #[cfg(not(feature = "bench"))]
                {
                    commands.entity(entity).insert(JoinTime(Instant::now()));
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
fn check_frames(mut commands: Commands, mut query: Query<(Entity, &mut Connection, &JoinTime)>) {
    'entities: for (entity, mut connection, join_time) in &mut query {
        while !connection.incoming_buf.is_empty() {
            let mut buf: Cursor<&[u8]> = Cursor::new(&connection.incoming_buf[..]);
            let len_before_packet: usize = buf.remaining(); // The length of the buffer before reading anything

            // Try to read the packet length
            match usize::read_var_from(&mut buf) {
                Ok(packet_length) => {
                    // Check that the packet length is valid
                    if packet_length < usize::MIN_SIZE {
                        trace!("Invalid packet length (0), despawning entity");
                        commands.entity(entity).despawn();
                        continue 'entities;
                    }

                    // Check if we have enough bytes to parse the packet
                    if packet_length > buf.remaining() {
                        continue 'entities;
                    }

                    // We know we have enough bytes, so it's safe to index into the buffer.
                    let mut packet_data: Cursor<&[u8]> = Cursor::new(
                        &buf.get_ref()
                            [buf.position() as usize..buf.position() as usize + packet_length],
                    );

                    let len_before_data: usize = buf.remaining(); // The length of the buffer before reading the packet data
                    let Ok(packet_id) = u32::read_var_from(&mut packet_data) else {
                        trace!("Invalid packet id, despawning entity");
                        commands.entity(entity).despawn();
                        continue 'entities;
                    };

                    // Parse the packet
                    match parse_packet(
                        &mut commands,
                        entity,
                        connection.state,
                        packet_id,
                        &mut packet_data,
                    ) {
                        Ok(can_change_state) => {
                            let len_after_packet: usize = packet_data.remaining(); // The length of the buffer after reading the packet

                            // Ensure that packet data is empty.
                            // If it's not, this means that packet length was invalid.
                            if len_after_packet != 0 {
                                trace!("Leftover packet data, despawning entity");
                                commands.entity(entity).despawn();
                                continue 'entities;
                            }

                            // Advance the buffer to the end of the packet
                            connection
                                .incoming_buf
                                .advance(len_before_packet - len_before_data + packet_length);

                            // If this packet can change the state, we can't continue parsing packets.
                            if can_change_state {
                                continue 'entities;
                            }
                        }
                        // This is redundant, because we know we have enough bytes to be able to parse the packet.
                        // Err(PacketReadError::Io(_)) => continue 'entities,
                        Err(e) => {
                            trace!("Parse error: {e:?}, despawning entity");
                            commands.entity(entity).despawn();
                            continue 'entities;
                        }
                    }
                }
                // The packet length was invalid
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

        #[cfg(feature = "bench")]
        {
            info!("Empty buffer, took: {:?}", join_time.0.elapsed());
            commands.entity(entity).despawn();
        }
    }
}

/// The current implementation will insert parse a packet and insert it as a component to the given entity.
/// It will return whether the parsed packet can change the state.
#[instrument(skip_all, level = "trace")]
fn parse_packet(
    commands: &mut Commands,
    entity: Entity,
    state: State,
    packet_id: u32,
    reader: &mut impl Read,
) -> Result<bool, PacketReadError> {
    packet_decoder_map!(
        commands, entity, state, packet_id, reader,
        State::Handshaking => (Handshake),
        State::Status => (StatusRequest, PingRequest),
        State::Login => (LoginStart, EncryptionResponse),
        _ => (),
    )
}

/// This function reads a packet of type T and dispatches it to `Events<PacketEvent<T>>`.
#[instrument(skip_all, level = "trace")]
fn dispath_packet<T: Packet + Send + Sync + 'static>(
    commands: &mut Commands,
    entity: Entity,
    reader: &mut impl Read,
) -> Result<(), PacketReadError> {
    let packet: T = T::read(reader)?;
    commands.add(move |world: &mut World| {
        world.send_event(PacketEvent { entity, packet });
    });
    Ok(())
}

#[instrument(skip_all, level = "trace")]
fn handle_handshake(
    mut query: Query<&mut Connection>,
    mut handshake_rx: EventReader<PacketEvent<Handshake>>,
) {
    for PacketEvent { entity, packet } in handshake_rx.read() {
        trace!("Handshake received, next state: {}", packet.next_state);
        if let Ok(mut connection) = query.get_mut(*entity) {
            connection.state = packet.next_state.into();
        }
    }
}

#[instrument(skip_all, level = "trace")]
fn handle_status_request(
    mut query: Query<&mut Connection>,
    mut status_request_rx: EventReader<PacketEvent<StatusRequest>>,
) {
    for PacketEvent { entity, .. } in status_request_rx.read() {
        trace!("Status request received");
        if let Ok(mut connection) = query.get_mut(*entity) {
            let buf: Vec<u8> = serialize_packet(StatusResponse {
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
        }
    }
}

#[instrument(skip_all, level = "trace")]
fn handle_ping_request(
    mut commands: Commands,
    mut query: Query<&mut Connection>,
    mut ping_request_rx: EventReader<PacketEvent<PingRequest>>,
) {
    for PacketEvent { entity, packet } in ping_request_rx.read() {
        trace!("Ping request received");
        if let Ok(mut connection) = query.get_mut(*entity) {
            let buf: Vec<u8> = serialize_packet(PingResponse {
                payload: packet.payload,
            });
            connection.stream.write_all(&buf).unwrap();
            commands.entity(*entity).despawn();
        }
    }
}

#[instrument(skip_all, level = "trace")]
fn handle_login_start(
    mut commands: Commands,
    mut query: Query<&mut Connection>,
    mut login_start_rx: EventReader<PacketEvent<LoginStart>>,
) {
    for PacketEvent { entity, packet } in login_start_rx.read() {
        trace!("Login start received");
        if let Ok(mut connection) = query.get_mut(*entity) {
            let buf: Vec<u8> = serialize_packet(Disconnect {
                reason: format!(
                    r#"
                {{
                    "text": "Fuck off, {}",
                    "color": "red",
                    "bold": true
                }}
                "#,
                    packet.username
                ),
            });
            connection.stream.write_all(&buf).unwrap();
            commands.entity(*entity).despawn();
        }
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
