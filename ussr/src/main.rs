mod init_tracing;

use std::{fs, io::Write, net::TcpStream};

use bevy_app::{App, PostStartup, ScheduleRunnerPlugin};
use tracing::{info, instrument, trace};
use ussr_net::{serialize_packet, UssrNetPlugin};

fn main() {
    init_tracing::init();

    #[cfg(feature = "server")]
    App::new()
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(UssrNetPlugin)
        .add_systems(PostStartup, post_startup)
        .run();

    let buf: Vec<u8> = fs::read("1_million_handshakes.bin").unwrap();
    trace!("Sending {} bytes", buf.len());
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:25565").unwrap();
    stream.write_all(&buf).unwrap();
}

#[instrument(skip_all, level = "trace")]
fn post_startup() {
    info!("Done!");
}
