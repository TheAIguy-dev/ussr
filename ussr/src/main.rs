mod init_tracing;

use bevy_app::{App, ScheduleRunnerPlugin};
use init_tracing::init_tracing;
use ussr_net::UssrNetPlugin;

fn main() {
    init_tracing();

    App::new()
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(UssrNetPlugin)
        .run();
}
