pub mod proto;

use std::io::{self, Read, Write};

use thiserror::Error;
use ussr_buf::ReadError;

use proto::enums::State;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Serverbound,
    Clientbound,
}

#[derive(Debug, Error)]
pub enum PacketReadError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Unknown packet id {packet_id} in state {state}")]
    UnknownPacketId { packet_id: u32, state: State },

    #[error("Couldn't parse packet: {0}")]
    Parse(ReadError),
}

impl From<ReadError> for PacketReadError {
    fn from(e: ReadError) -> Self {
        match e {
            ReadError::Io(e) => Self::Io(e),
            e => Self::Parse(e),
        }
    }
}

pub trait Packet: Sized {
    /// The packet id.
    const ID: u32;

    /// The packet direction.
    const DIRECTION: Direction;

    /// The connection state in which this packet is received/sent.
    const STATE: State;

    /// Whether this packet can change the connection state.
    const CAN_CHANGE_STATE: bool = false;

    /// The minimum size of the packet in bytes when serialized.
    const MIN_SIZE: usize;

    /// The maximum size of the packet in bytes when serialized.
    const MAX_SIZE: usize;

    /// Reads the packet from the given reader.
    fn read_from(reader: &mut impl Read) -> Result<Self, PacketReadError>;

    /// Writes the packet to the given writer.
    fn write_to(&self, writer: &mut impl Write) -> io::Result<()>;
}
