#![allow(unused)]
// Sample generated code from SpecMC

use std::{
    fmt::Write,
    io::{self, Read},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WritePacketError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("Connection closed")]
    ConnectionClosed,
}

#[derive(Debug, Error)]
pub enum ReadPacketError {
    #[error(transparent)]
    IoError(io::Error),
    #[error("Unknown packet id: {0}")]
    UnknownPacketId(u32),
    #[error("Parse error: {0}")]
    ParseError(&'static str),
    #[error("Connection closed")]
    ConnectionClosed,
}

pub trait Packet: Sized {
    /// Get the packet id.
    fn id() -> u32;

    /// Read the packet from the given reader.
    fn read(_: &mut impl Read) -> Result<Self, WritePacketError>;

    /// Write the packet to the given writer.
    fn write(&self, _: &mut impl Write) -> Result<(), WritePacketError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextState {
    Status = 1,
    Login = 2,
}

pub mod packets {
    pub mod handshake {
        pub mod serverbound {
            use super::super::super::*;

            pub struct Handshake {
                protocol_version: i32,
                server_address: String,
                server_port: u16,
                next_state: NextState,
            }
        }
        pub mod clientbound {}
    }
}
