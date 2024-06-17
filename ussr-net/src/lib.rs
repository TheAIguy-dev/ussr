mod handlers;
mod process_data;

use std::{
    io::{self, ErrorKind, Read},
    net::{TcpListener, TcpStream},
};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bytes::BytesMut;
use tracing::{instrument, trace, warn};
use ussr_buf::VarWritable;
use ussr_protocol::{
    proto::{
        enums::State,
        packets::{handshaking::serverbound::*, login::serverbound::*, status::serverbound::*},
    },
    Packet,
};

use process_data::process_data;

pub struct UssrNetPlugin;
impl Plugin for UssrNetPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_event::<PacketEvent<Handshake>>();
        app.add_event::<PacketEvent<StatusRequest>>();
        app.add_event::<PacketEvent<PingRequest>>();
        app.add_event::<PacketEvent<LoginStart>>();

        use handlers::{handshaking::*, login::*, status::*};
        app.insert_resource(Server::new()).add_systems(
            Update,
            (
                accept_connections,
                read_data,
                process_data,
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
struct Connection {
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

#[derive(Event)]
struct PacketEvent<T: Packet> {
    entity: Entity,
    packet: T,
}

/// A system that accepts connections and spawns new entities with [`Connection`].
/// [`Server::listener`] must be non-blocking, see [`TcpListener::set_nonblocking`].
#[instrument(skip_all, level = "trace")]
fn accept_connections(mut commands: Commands, server: Res<Server>) {
    while let Ok((stream, _)) = server.listener.accept() {
        trace!("Accepted connection");
        commands.spawn(Connection::new(stream).expect("Failed to create connection"));
    }
}

/// A system that reads data from a [`Connection`].
/// [`Connection::stream`] must be non-blocking, see [`TcpStream::set_nonblocking`].
#[instrument(skip_all, level = "trace")]
fn read_data(mut commands: Commands, mut query: Query<(Entity, &mut Connection)>) {
    let mut buf: Vec<u8> = vec![0; READ_BUFFER_SIZE];

    for (entity, mut connection) in &mut query {
        match connection.stream.read(&mut buf) {
            // Connection closed
            Ok(0) => {
                trace!("Connection closed");
                //? Maybe we shouldn't despawn the entity, and process the leftower data?
                commands.entity(entity).despawn();
                // commands.entity(entity).remove::<Connection>();
            }

            // Successful read
            Ok(n) => {
                trace!("Read {n} bytes");
                connection.incoming_buf.extend_from_slice(&buf[..n]);
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
