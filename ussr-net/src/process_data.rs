use std::io::{Cursor, Read};

use bevy_ecs::prelude::*;
use bytes::Buf;
use tracing::{instrument, trace, warn};
use ussr_buf::{ReadError, VarReadable, VarSize};
use ussr_protocol::{
    proto::{
        enums::{NextState, State},
        packets::{handshaking::serverbound::*, login::serverbound::*, status::serverbound::*},
    },
    Packet, PacketReadError,
};

use crate::{Connection, PacketEvent};

#[instrument(skip_all, level = "trace")]
pub(crate) fn process_data(mut commands: Commands, mut query: Query<(Entity, &mut Connection)>) {
    'entities: for (entity, mut connection) in &mut query {
        while !connection.incoming_buf.is_empty() {
            let mut buf: Cursor<&[u8]> = Cursor::new(&connection.incoming_buf[..]);
            let len_before_packet: usize = buf.remaining(); // The length of the buffer before reading anything

            // Try to read the packet length
            match usize::read_var_from(&mut buf) {
                Ok(packet_length) => {
                    // Check that the packet length is valid
                    if packet_length < usize::MIN_SIZE {
                        trace!("Invalid packet length (0), despawning entity");
                        commands.entity(entity).despawn();
                        continue 'entities;
                    }

                    // Check if we have enough bytes to parse the packet
                    if packet_length > buf.remaining() {
                        continue 'entities;
                    }

                    // We know we have enough bytes, so it's safe to index into the buffer.
                    let mut packet_data: Cursor<&[u8]> = Cursor::new(
                        &buf.get_ref()
                            [buf.position() as usize..buf.position() as usize + packet_length],
                    );

                    let len_before_data: usize = buf.remaining(); // The length of the buffer before reading the packet data
                    let Ok(packet_id) = u32::read_var_from(&mut packet_data) else {
                        trace!("Invalid packet id, despawning entity");
                        commands.entity(entity).despawn();
                        continue 'entities;
                    };

                    // Parse the packet
                    match parse_packet(
                        &mut commands,
                        entity,
                        connection.state,
                        packet_id,
                        &mut packet_data,
                    ) {
                        Ok(state) => {
                            let len_after_packet: usize = packet_data.remaining(); // The length of the buffer after reading the packet

                            // Ensure that packet data is empty.
                            // If it's not, this means that packet length was invalid.
                            if len_after_packet != 0 {
                                trace!("Leftover packet data, despawning entity");
                                commands.entity(entity).despawn();
                                continue 'entities;
                            }

                            // Advance the buffer to the end of the packet
                            connection
                                .incoming_buf
                                .advance(len_before_packet - len_before_data + packet_length);

                            // Update the connection state
                            if let Some(state) = state {
                                connection.state = state;
                            }
                        }
                        // This is redundant, because we know we have enough bytes to be able to parse the packet.
                        // Err(PacketReadError::Io(_)) => continue 'entities,
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
    commands: &mut Commands,
    entity: Entity,
    state: State,
    packet_id: u32,
    reader: &mut impl Read,
) -> Result<Option<State>, PacketReadError> {
    match state {
        State::Handshaking => match packet_id {
            Handshake::ID => {
                trace!("Reading handshake");
                let p: Handshake = Handshake::read(reader)?;
                let next_state: NextState = p.next_state;
                dispath_packet::<Handshake>(commands, entity, p)?;
                Ok(Some(next_state.into()))
            }
            _ => return Err(PacketReadError::UnknownPacketId { packet_id, state }),
        },
        State::Status => match packet_id {
            StatusRequest::ID => {
                trace!("Reading status request");
                dispath_packet::<StatusRequest>(commands, entity, StatusRequest::read(reader)?)?;
                Ok(None)
            }
            PingRequest::ID => {
                trace!("Reading ping request");
                dispath_packet::<PingRequest>(commands, entity, PingRequest::read(reader)?)?;
                Ok(None)
            }
            _ => return Err(PacketReadError::UnknownPacketId { packet_id, state }),
        },
        State::Login => match packet_id {
            LoginStart::ID => {
                trace!("Reading login start");
                dispath_packet::<LoginStart>(commands, entity, LoginStart::read(reader)?)?;
                Ok(None)
            }
            EncryptionResponse::ID => {
                trace!("Reading encryption response");
                dispath_packet::<EncryptionResponse>(
                    commands,
                    entity,
                    EncryptionResponse::read(reader)?,
                )?;
                Ok(None)
            }
            _ => return Err(PacketReadError::UnknownPacketId { packet_id, state }),
        },
        _ => match packet_id {
            _ => return Err(PacketReadError::UnknownPacketId { packet_id, state }),
        },
    }
}

/// This function dispatches a packet of type `T` as an event.
#[instrument(skip_all, level = "trace")]
fn dispath_packet<P>(
    commands: &mut Commands,
    entity: Entity,
    packet: P,
) -> Result<(), PacketReadError>
where
    P: Packet + Send + Sync + 'static,
{
    commands.add(move |world: &mut World| {
        world.send_event(PacketEvent { entity, packet });
    });
    Ok(())
}
