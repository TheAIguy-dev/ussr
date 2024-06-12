use std::{
    io::{self, Cursor, Read},
    net::{TcpListener, TcpStream},
};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bytes::{Buf, BytesMut};
use tracing::{instrument, trace};
use ussr_buf::VarReadable;

pub struct UssrNetPlugin;
impl Plugin for UssrNetPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.insert_resource(Server::new())
            .add_systems(Update, (accept_connections, read_data, test).chain());
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
        let listener: TcpListener = TcpListener::bind("127.0.0.1:25565").unwrap();
        listener.set_nonblocking(true).unwrap();
        Self { listener }
    }
}

/// A single connection to the server.
/// This component is added automatically by [`accept_connections`].
#[derive(Component)]
pub struct Connection {
    stream: TcpStream,
    buf: BytesMut,
}
impl Connection {
    /// Create a new connection.
    /// The stream will be made non-blocking, see [`TcpStream::set_nonblocking`].
    fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self {
            stream,
            buf: BytesMut::new(),
        })
    }
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
                trace!("Connection closed");
                commands.entity(entity).remove::<Connection>();
            }
            // Successful read
            Ok(n) => {
                trace!("Read {n} bytes");
                connection.buf.extend_from_slice(&buf[..n])
            }
            // Wood block
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            // Other error
            Err(e) => {
                trace!("Error: {e:?}");
                continue;
            }
        }
    }
}

#[instrument(skip_all, level = "trace")]
fn test(mut query: Query<&mut Connection>) {
    for mut connection in &mut query {
        if !connection.buf.is_empty() {
            let mut buf = Cursor::new(&connection.buf);
            trace!("Buffer: {:?}", connection.buf);
            if let Ok(packet_length) = usize::read_var_from(&mut buf) {
                trace!("Packet length: {packet_length}");
                if packet_length <= buf.remaining() {
                    let packet_length_length: usize = connection.buf.len() - buf.remaining();
                    trace!(
                        "Packet data: {:?}",
                        &buf.get_ref()[buf.position() as usize..packet_length]
                    );
                    connection.buf.advance(packet_length_length + packet_length);
                }
            }
        }
    }
}
