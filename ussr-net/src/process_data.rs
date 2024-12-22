use std::io::{Cursor, Read};

use bevy_ecs::prelude::*;
use bytes::{Buf, BytesMut};
use tracing::{instrument, trace, warn};
use ussr_buf::{DecodeError, VarDecode};
use ussr_protocol::PacketDecodeError;

use crate::{Connection, MAX_PACKET_SIZE};

#[instrument(skip_all, level = "trace")]
pub(crate) fn process_data(mut commands: Commands, mut query: Query<(Entity, &mut Connection)>) {
    'entities: for (entity, mut connection) in &mut query {
        while !connection.incoming_buf.is_empty() {
            let mut buf: Cursor<&[u8]> = Cursor::new(&connection.incoming_buf[..]);
            let len_before_length = buf.remaining(); // Length of the buffer before reading packet length

            // Try to read the packet length
            match usize::var_decode(&mut buf) {
                Ok(packet_length) => {
                    let len_after_length = buf.remaining();

                    // Check that the packet length is valid
                    if packet_length > MAX_PACKET_SIZE {
                        trace!("Invalid packet length, despawning entity");
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

                    let Ok(packet_id) = u32::var_decode(&mut packet_data) else {
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
                Err(DecodeError::InvalidVarInt) => {
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
) -> Result<(), PacketDecodeError> {
    let _ = (connection, packet_id, reader);
    todo!()
}
