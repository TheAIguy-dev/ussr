use std::io::{Cursor, Read};

use bevy_ecs::prelude::*;
use bytes::{Buf, BytesMut};
use tracing::{instrument, trace, warn};
use ussr_buf::{ReadError, VarReadable, VarSize};
use ussr_protocol::{
    proto::{
        enums::{NextState, State},
        packets::{
            handshaking::serverbound::*,
            login::{clientbound::Disconnect, serverbound::*},
            status::{
                clientbound::{PingResponse, StatusResponse},
                serverbound::*,
            },
        },
    },
    Packet, PacketReadError,
};

use crate::{serialize_packet, Connection};

#[instrument(skip_all, level = "trace")]
pub(crate) fn process_data(mut commands: Commands, mut query: Query<(Entity, &mut Connection)>) {
    'entities: for (entity, mut connection) in &mut query {
        while !connection.incoming_buf.is_empty() {
            let mut buf: Cursor<&[u8]> = Cursor::new(&connection.incoming_buf[..]);
            let len_before_length = buf.remaining(); // Length of the buffer before reading packet length

            // Try to read the packet length
            match usize::read_var_from(&mut buf) {
                Ok(packet_length) => {
                    let len_after_length = buf.remaining();

                    // Check that the packet length is valid
                    if packet_length < usize::MIN_SIZE
                        || packet_length > 0b111_1111__111_1111__111_1111
                    {
                        trace!("Invalid packet length (0), despawning entity");
                        commands.entity(entity).despawn();
                        continue 'entities;
                    }

                    // Check if we have enough bytes to parse the packet
                    if packet_length > len_after_length {
                        continue 'entities;
                    }

                    // Remove the packet length from the buffer
                    connection
                        .incoming_buf
                        .advance(len_before_length - len_after_length);

                    // Split the buffer into the packet data and the rest
                    let packet_data: BytesMut = connection.incoming_buf.split_to(packet_length);
                    let mut packet_data: Cursor<&[u8]> = Cursor::new(&packet_data[..]);

                    let Ok(packet_id) = u32::read_var_from(&mut packet_data) else {
                        trace!("Invalid packet id, despawning entity");
                        commands.entity(entity).despawn();
                        continue 'entities;
                    };

                    // Parse the packet
                    match parse_packet(
                        // &mut commands,
                        // entity,
                        &mut connection,
                        packet_id,
                        &mut packet_data,
                    ) {
                        Ok(_) => {
                            // Ensure that packet data is empty.
                            // If it's not, this means that packet length was invalid.
                            if packet_data.remaining() != 0 {
                                trace!("Leftover packet data, despawning entity");
                                commands.entity(entity).despawn();
                                continue 'entities;
                            }

                            // // Update the connection state
                            // if let Some(state) = state {
                            //     connection.state = state;
                            // }
                        }

                        Err(e) => {
                            trace!("Parse error: {e:?}, despawning entity");
                            commands.entity(entity).despawn();
                            continue 'entities;
                        }
                    }
                }

                // The packet length was invalid
                Err(ReadError::InvalidVarInt) => {
                    trace!("Invalid packet length, despawning entity");
                    commands.entity(entity).despawn();
                    continue 'entities;
                }

                // Every other error is an IO error.
                // This means that we don't have enough bytes to read the packet length.
                Err(_) => continue 'entities,
            }
        }
    }
}

/// This function will parse and dispatch a packet as a [`PacketEvent`].
/// It will return a state to transition to.
#[instrument(skip_all, level = "trace")]
fn parse_packet(
    // commands: &mut Commands,
    // entity: Entity,
    connection: &mut Connection,
    packet_id: u32,
    reader: &mut impl Read,
) -> Result<(), PacketReadError> {
    match connection.state {
        State::Handshaking => match packet_id {
            Handshake::ID => {
                trace!("Reading handshake");
                let p: Handshake = Handshake::read_from(reader)?;
                let next_state: NextState = p.next_state;
                connection.state = next_state.into();
            }
            _ => {
                return Err(PacketReadError::UnknownPacketId {
                    packet_id,
                    state: connection.state,
                })
            }
        },
        State::Status => match packet_id {
            StatusRequest::ID => {
                trace!("Reading status request");
                let p = StatusRequest::read_from(reader)?;
                let buf: Vec<u8> = serialize_packet(StatusResponse {
                    response: r#"{"version":{"name":"1.7.2","protocol":4},"players":{"max":100,"online":0,"sample":[]},"description":{"text":"Hello, world!"}}"#.to_string(),
                });
                connection.outgoing_buf.extend_from_slice(&buf);
            }
            PingRequest::ID => {
                trace!("Reading ping request");
                let p = PingRequest::read_from(reader)?;
                let buf: Vec<u8> = serialize_packet(PingResponse { payload: p.payload });
                connection.outgoing_buf.extend_from_slice(&buf);
            }
            _ => {
                return Err(PacketReadError::UnknownPacketId {
                    packet_id,
                    state: connection.state,
                })
            }
        },
        State::Login => match packet_id {
            LoginStart::ID => {
                trace!("Reading login start");
                let p = LoginStart::read_from(reader)?;
                let buf: Vec<u8> = serialize_packet(Disconnect {
                    reason: format!(
                        r#"{{"text": "Fuck off, {}","color": "red","bold": true}}"#,
                        p.username
                    ),
                });
                connection.outgoing_buf.extend_from_slice(&buf);
            }
            EncryptionResponse::ID => {
                trace!("Reading encryption response");
                let p = EncryptionResponse::read_from(reader)?;
            }
            _ => {
                return Err(PacketReadError::UnknownPacketId {
                    packet_id,
                    state: connection.state,
                })
            }
        },
        _ => match packet_id {
            _ => {
                return Err(PacketReadError::UnknownPacketId {
                    packet_id,
                    state: connection.state,
                })
            }
        },
    }

    Ok(())
}
