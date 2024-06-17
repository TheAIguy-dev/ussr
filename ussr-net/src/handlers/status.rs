use std::io::Write;

use bevy_ecs::prelude::*;
use tracing::{instrument, trace, warn};
use ussr_protocol::proto::packets::status::{
    clientbound::{PingResponse, StatusResponse},
    serverbound::{PingRequest, StatusRequest},
};

use crate::{serialize_packet, Connection, PacketEvent};

#[instrument(skip_all, level = "trace")]
pub(crate) fn handle_status_request(
    mut query: Query<&mut Connection>,
    mut status_request_rx: EventReader<PacketEvent<StatusRequest>>,
) {
    for PacketEvent { entity, .. } in status_request_rx.read() {
        trace!("Status request received");
        if let Ok(mut connection) = query.get_mut(*entity) {
            let buf: Vec<u8> = serialize_packet(StatusResponse {
                response: r#"
            {
                "version": {
                    "name": "1.7.2",
                    "protocol": 4
                },
                "players": {
                    "max": 100,
                    "online": 0,
                    "sample": []
                },
                "description": {
                    "text": "Hello, world!"
                }
            }
            "#
                .to_string(),
            });
            connection.stream.write_all(&buf).unwrap();
        }
    }
}

#[instrument(skip_all, level = "trace")]
pub(crate) fn handle_ping_request(
    mut commands: Commands,
    mut query: Query<&mut Connection>,
    mut ping_request_rx: EventReader<PacketEvent<PingRequest>>,
) {
    for PacketEvent { entity, packet } in ping_request_rx.read() {
        trace!("Ping request received");
        if let Ok(mut connection) = query.get_mut(*entity) {
            let buf: Vec<u8> = serialize_packet(PingResponse {
                payload: packet.payload,
            });
            connection.stream.write_all(&buf).unwrap();
            commands.entity(*entity).despawn();
        }
    }
}
