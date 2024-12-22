mod versions;

use std::io;

use thiserror::Error;
use ussr_buf::DecodeError;

pub use enums::State;
pub use versions::*;

//? A possible future optimization is to make packets immutable (e.g. use `Box<[T]>` instead of `Vec<T>`)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Serverbound,
    Clientbound,
}

#[derive(Debug, Error)]
pub enum PacketDecodeError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Unknown packet id {packet_id} in state {state:?}")]
    UnknownPacketId { packet_id: u32, state: State },

    #[error("Couldn't decode packet: {0}")]
    Decode(DecodeError),
}

impl From<DecodeError> for PacketDecodeError {
    fn from(value: DecodeError) -> Self {
        match value {
            DecodeError::Io(e) => PacketDecodeError::Io(e),
            e => PacketDecodeError::Decode(e),
        }
    }
}
