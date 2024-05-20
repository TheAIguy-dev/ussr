use std::{
    io::{self, Read},
    net::{TcpListener, TcpStream},
};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

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
    buf: Vec<u8>,
}
impl Connection {
    /// Create a new connection.
    /// The stream will be made non-blocking, see [`TcpStream::set_nonblocking`].
    fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self {
            stream,
            buf: vec![],
        })
    }
}

/// A system that accepts connections and spawns new entities with [`Connection`].
/// The listener must be non-blocking, see [`TcpListener::set_nonblocking`].
fn accept_connections(mut commands: Commands, server: Res<Server>) {
    loop {
        match server.listener.accept() {
            Ok((stream, _)) => commands.spawn(match Connection::new(stream) {
                Ok(connection) => {println!("Accepted connection");connection},
                Err(_) => return,
            }),
            Err(_) /* if err.kind() == io::ErrorKind::WouldBlock */ => return,
            // Err(err) => return Err(err),
        };
    }
}

/// A system that reads data from a [`Connection`].
///
/// The connection must be non-blocking, see [`TcpStream::set_nonblocking`].
fn read_data(mut commands: Commands, mut query: Query<(Entity, &mut Connection)>) {
    let mut buf: Vec<u8> = vec![0; READ_BUFFER_SIZE];

    for (entity, mut connection) in &mut query {
        match connection.stream.read(&mut buf) {
            // Connection closed
            Ok(0) => {
                println!("Connection closed");
                commands.entity(entity).remove::<Connection>();
            }
            // Successful read
            Ok(n) => {println!("Read {} bytes", n);connection.buf.extend_from_slice(&buf[..n])},
            // Wood block
            Err(_) /* if err.kind() == io::ErrorKind::WouldBlock */ => continue,
            // Other error
            // Err(err) => return Err(err),
        };
    }
}

fn test(query: Query<&Connection>) {
    for connection in &query {
        println!("{:?}", connection.buf);
    }
}
