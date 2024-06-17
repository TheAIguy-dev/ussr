mod init_tracing;

use std::{fs, io::Write, net::TcpStream};

use bevy_app::{App, PostStartup, ScheduleRunnerPlugin};
use tracing::{info, instrument, trace};
use ussr_net::{serialize_packet, UssrNetPlugin};
use ussr_protocol::proto::{
    enums::NextState,
    packets::{handshaking::serverbound::Handshake, status::serverbound::StatusRequest},
};

fn main() {
    init_tracing::init();

    #[cfg(feature = "server")]
    App::new()
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(UssrNetPlugin)
        .add_systems(PostStartup, post_startup)
        .run();

    // let buf: Vec<u8> = [
    //     serialize_packet(Handshake {
    //         protocol_version: 4,
    //         server_address: "localhost".to_string(),
    //         server_port: 25565,
    //         next_state: NextState::Status,
    //     }),
    //     serialize_packet(StatusRequest {}).repeat(10_000_000),
    // ]
    // .concat();
    // fs::write("10_000_000_status_requests.bin", &buf).unwrap();

    let buf: Vec<u8> = fs::read("10_000_000_status_requests.bin").unwrap();
    trace!("Sending {} bytes", buf.len());
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:25565").unwrap();
    stream.write_all(&buf).unwrap();
}

#[instrument(skip_all, level = "trace")]
fn post_startup() {
    info!("Done!");
}
