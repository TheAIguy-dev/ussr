mod init_tracing;

use bevy_app::{App, PostStartup, ScheduleRunnerPlugin};
use tracing::{info, instrument};
use ussr_net::UssrNetPlugin;

fn main() {
    init_tracing::init();

    App::new()
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(UssrNetPlugin)
        .add_systems(PostStartup, post_startup)
        .run();

    // const ITERATIONS: usize = 1_000_000;

    // let packet: Handshake = Handshake {
    //     protocol_version: 765,
    //     server_address: "hola!".to_string(),
    //     server_port: 25565,
    //     next_state: NextState::Status,
    // };
    // let mut buf = vec![];
    // packet.write(&mut buf).unwrap();
    // let mut buf = buf.repeat(ITERATIONS);

    // let mut packets: Vec<Handshake> = Vec::with_capacity(ITERATIONS);
    // let start: Instant = Instant::now();

    // for _ in 0..ITERATIONS {
    //     packets.push(Handshake::read(&mut Cursor::new(&buf)).unwrap());
    // }

    // println!("{:?}", start.elapsed());
    // println!("{:?}", buf.is_empty());
}

#[instrument(skip_all, level = "trace")]
fn post_startup() {
    info!("Done!");
}
