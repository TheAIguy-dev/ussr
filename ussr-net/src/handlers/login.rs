use std::io::Write;

use bevy_ecs::prelude::*;
use tracing::{instrument, trace, warn};
use ussr_protocol::proto::packets::login::{clientbound::Disconnect, serverbound::LoginStart};

use crate::{serialize_packet, Connection, PacketEvent};

#[instrument(skip_all, level = "trace")]
pub(crate) fn handle_login_start(
    mut commands: Commands,
    mut query: Query<&mut Connection>,
    mut login_start_rx: EventReader<PacketEvent<LoginStart>>,
) {
    for PacketEvent { entity, packet } in login_start_rx.read() {
        trace!("Login start received");
        if let Ok(mut connection) = query.get_mut(*entity) {
            let buf: Vec<u8> = serialize_packet(Disconnect {
                reason: format!(
                    r#"
                {{
                    "text": "Fuck off, {}",
                    "color": "red",
                    "bold": true
                }}
                "#,
                    packet.username
                ),
            });
            connection.stream.write_all(&buf).unwrap();
            commands.entity(*entity).despawn();
        }
    }
}
