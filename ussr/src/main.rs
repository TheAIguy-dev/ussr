use std::io::Write;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use chrono::Local;
use env_logger::Builder;
use eyre::Result; //? I probably shouldn't use this.
use log::LevelFilter;

// Very important
static HEROBRINE: &str = "herobrine";

// fn main() -> Result<()> {
fn main() {
    // color_eyre::install()?;

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    if false {
        println!("{HEROBRINE}");
    }

    // server::start()
    let mut world: World = World::new();

    // Ok(())
}
