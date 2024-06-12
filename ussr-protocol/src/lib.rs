mod generated;

use std::io::{self, Read, Write};

use thiserror::Error;
use ussr_buf::ReadError;

use generated::enums::State;

pub enum Direction {
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
    const DIRECTION: Direction;

    /// The state in which this packet is received/sent.
    const STATE: State;

    fn read(buf: &mut impl Read) -> Result<Self, PacketReadError>;

    fn write(&self, buf: &mut impl Write) -> io::Result<()>;
}
