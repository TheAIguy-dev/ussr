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
}

#[instrument(skip_all, level = "trace")]
fn post_startup() {
    info!("Done!");
}
