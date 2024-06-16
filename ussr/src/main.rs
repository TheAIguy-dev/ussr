mod init_tracing;

use std::{fs, io::Write, net::TcpStream};

use bevy_app::{App, PostStartup, ScheduleRunnerPlugin};
use init_tracing::init_tracing;
use tracing::{info, instrument};
use ussr_net::{serialize_packet, UssrNetPlugin};

fn main() {
    init_tracing();

    #[cfg(feature = "server")]
    App::new()
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(UssrNetPlugin)
        .add_systems(PostStartup, post_startup)
        .run();

    let buf: Vec<u8> = fs::read("1_million_handshakes.bin").unwrap();
    info!("Sending {} bytes", buf.len());
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:25565").unwrap();
    stream.write_all(&buf).unwrap();
}

#[instrument(skip_all, level = "trace")]
fn post_startup() {
    info!("Done!");
}
