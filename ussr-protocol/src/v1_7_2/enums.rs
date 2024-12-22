use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};
#[cfg(feature = "async")]
use futures_lite::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async")]
use ussr_buf::{AsyncDecode, AsyncEncode, AsyncEncodeExt};
use ussr_buf::{Decode, DecodeError, Encode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

impl From<NextState> for State {
    fn from(next_state: NextState) -> Self {
        match next_state {
            NextState::Status => State::Status,
            NextState::Login => State::Login,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NextState {
    Status = 1,
    Login = 2,
}

impl TryFrom<u8> for NextState {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            _ => Err(DecodeError::InvalidEnumVariant),
        }
    }
}

impl Decode for NextState {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        reader.read_u8()?.try_into()
    }
}

impl Encode for NextState {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(*self as u8)
    }
}

#[cfg(feature = "async")]
impl AsyncDecode for NextState {
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        <u8 as AsyncDecode>::decode(reader).await?.try_into()
    }
}

#[cfg(feature = "async")]
impl AsyncEncode for NextState {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.encode(*self as u8).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
}

impl TryFrom<u8> for Gamemode {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Gamemode::Survival),
            1 => Ok(Gamemode::Creative),
            2 => Ok(Gamemode::Adventure),
            _ => Err(DecodeError::InvalidEnumVariant),
        }
    }
}

impl Decode for Gamemode {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        reader.read_u8()?.try_into()
    }
}

impl Encode for Gamemode {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(*self as u8)
    }
}

#[cfg(feature = "async")]
impl AsyncDecode for Gamemode {
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        <u8 as AsyncDecode>::decode(reader).await?.try_into()
    }
}

#[cfg(feature = "async")]
impl AsyncEncode for Gamemode {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.encode(*self as u8).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Nether = -1,
    Overworld = 0,
    End = 1,
}

impl TryFrom<i8> for Dimension {
    type Error = DecodeError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Dimension::Nether),
            0 => Ok(Dimension::Overworld),
            1 => Ok(Dimension::End),
            _ => Err(DecodeError::InvalidEnumVariant),
        }
    }
}

impl Decode for Dimension {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        reader.read_i8()?.try_into()
    }
}

impl Encode for Dimension {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_i8(*self as i8)
    }
}

#[cfg(feature = "async")]
impl AsyncDecode for Dimension {
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        <i8 as AsyncDecode>::decode(reader).await?.try_into()
    }
}

#[cfg(feature = "async")]
impl AsyncEncode for Dimension {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.encode(*self as i8).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

impl TryFrom<u8> for Difficulty {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Difficulty::Peaceful),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Normal),
            3 => Ok(Difficulty::Hard),
            _ => Err(DecodeError::InvalidEnumVariant),
        }
    }
}

impl Decode for Difficulty {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        reader.read_u8()?.try_into()
    }
}

impl Encode for Difficulty {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(*self as u8)
    }
}

#[cfg(feature = "async")]
impl AsyncDecode for Difficulty {
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        <u8 as AsyncDecode>::decode(reader).await?.try_into()
    }
}

#[cfg(feature = "async")]
impl AsyncEncode for Difficulty {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.encode(*self as u8).await
    }
}
