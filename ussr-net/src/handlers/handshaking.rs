use bevy_ecs::prelude::*;
use tracing::{instrument, trace, warn};
use ussr_protocol::proto::packets::handshaking::serverbound::*;

use crate::PacketEvent;

#[instrument(skip_all, level = "trace")]
pub(crate) fn handle_handshake(mut handshake_rx: EventReader<PacketEvent<Handshake>>) {
    for PacketEvent { packet, .. } in handshake_rx.read() {
        trace!("Handshake received, next state: {}", packet.next_state);
        //? This is removed because `check_frames` already updates the state
        // if let Ok(mut connection) = query.get_mut(*entity) {
        //     connection.state = packet.next_state.into();
        // }
    }
}
