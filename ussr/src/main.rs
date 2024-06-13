mod init_tracing;

use bevy_app::{App, PostStartup, ScheduleRunnerPlugin};
use init_tracing::init_tracing;
use tracing::trace;
use ussr_net::UssrNetPlugin;

fn main() {
    init_tracing();

    App::new()
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(UssrNetPlugin)
        .add_systems(PostStartup, post_startup)
        .run();
}

fn post_startup() {
    trace!("Done!");
}
