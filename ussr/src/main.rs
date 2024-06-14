mod init_tracing;

use std::mem::size_of;

use bevy_app::{App, PostStartup, ScheduleRunnerPlugin};
use init_tracing::init_tracing;
use tracing::{instrument, trace};
use ussr_net::UssrNetPlugin;

macro_rules! print_each_field {
    (
        $(#[$ty_meta:meta])*
        $ty_vis:vis struct $ty_name:ident $([ $($generics:tt)* ])? {
            $(
                $( $(@$tag:tt)? #[tag] )? $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$ty_meta])*
        $ty_vis struct $ty_name $(< $($generics)* >)? {
            $( $field_name : $field_ty ),*
        }
        impl $(< $($generics)* >)? $ty_name $(< $($generics)* >)? {
            fn print_each_field(&self) {
                $(
                    choose!(
                        { $( $(@$tag)? + )? }
                        { println!(concat!(stringify!($field_name), ": {} with tag"), self.$field_name) }
                        { println!(concat!(stringify!($field_name), ": {}"), self.$field_name) }
                    );
                )*
            }
        }
    }
}
macro_rules! choose {
    ( {+} {$($then:tt)*} {$($else:tt)*} ) => { $($then)* };
    ( {} {$($then:tt)*} {$($else:tt)*} ) => { $($else)* }
}

fn main() {
    #[cfg(debug_assertions)]
    {
        print_each_field!(
            #[derive(Debug)]
            struct Packet {
                #[tag] id: u32,
                data: String
            }
        );
        Packet { id: 1, data: "hello".to_string() }.print_each_field();
    }

    init_tracing();

    App::new()
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(UssrNetPlugin)
        .add_systems(PostStartup, post_startup)
        .run();
}

#[instrument(skip_all, level = "trace")]
fn post_startup() {
    trace!("Done!");
}
