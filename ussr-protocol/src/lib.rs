mod versions;

use std::io::{self, Read, Write};

use thiserror::Error;
use ussr_buf::ReadError;

use versions::enums::State;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    Serverbound,
    Clientbound,
}

//? Maybe I don't need to implement Error.
#[derive(Debug, Error)]
pub enum PacketReadError {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("Unknown packet id {packet_id} in state {state}")]
    UnknownPacketId { packet_id: u32, state: State },

    #[error("Couldn't parse packet: {0}")]
    Parse(#[from] ReadError),
}

pub trait Packet: Sized {
    /// The packet id.
    const ID: u32;

    /// The packet direction.
    const DIRECTION: PacketDirection;

    /// The state in which this packet is received/sent.
    const STATE: State;

    /// The minimum size of the packet in bytes when serialized.
    const MIN_SIZE: usize;

    /// The maximum size of the packet in bytes when serialized.
    const MAX_SIZE: usize;

    /// Reads the packet from the given reader.
    fn read(reader: &mut impl Read) -> Result<Self, PacketReadError>;

    /// Writes the packet to the given writer.
    /// Note that this still performs validation, and will panic if the packet is invalid.
    fn write(&self, writer: &mut impl Write) -> io::Result<()>;
}
