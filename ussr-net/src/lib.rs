mod process_data;

use std::{
    io::{self, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bytes::BytesMut;
use tracing::{instrument, trace, warn};
use ussr_buf::VarWritable;
use ussr_protocol::{proto::enums::State, Packet};

use process_data::process_data;

pub struct UssrNetPlugin;
impl Plugin for UssrNetPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.insert_resource(Server::new()).add_systems(
            Update,
            (accept_connections, read_data, process_data, send_data).chain(),
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
    outgoing_buf: Vec<u8>,
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
            outgoing_buf: vec![],
        })
    }
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

#[instrument(skip_all, level = "trace")]
fn send_data(mut commands: Commands, mut query: Query<(Entity, &mut Connection)>) {
    for (entity, mut connection) in &mut query {
        if connection.outgoing_buf.is_empty() {
            continue;
        }

        let mut outgoing_buf: Vec<u8> = std::mem::take(&mut connection.outgoing_buf);
        match connection.stream.write(&outgoing_buf) {
            // Successful write
            Ok(n) => {
                trace!("Wrote {n} bytes");
                std::mem::swap(&mut connection.outgoing_buf, &mut outgoing_buf);
                connection.outgoing_buf.drain(..n);
            }

            // Wood block
            Err(e) if e.kind() == ErrorKind::WouldBlock => continue,

            // Other error
            Err(e) => {
                warn!("Error: {e:?}");
                commands.entity(entity).despawn();
            }
        }
    }
}

// TODO: system for writing outgoing data
// TODO: function to prefix the packet with its length
