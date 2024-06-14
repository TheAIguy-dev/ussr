use std::io::{self, Read, Write};

use strum_macros::Display;
use ussr_buf::{ReadError, Readable, Size, VarReadable, VarSize, VarWritable, Writable};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NextState {
    Status = 1,
    Login = 2,
}
impl TryFrom<i32> for NextState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            _ => Err(()),
        }
    }
}
impl Into<State> for NextState {
    fn into(self) -> State {
        match self {
            Self::Status => State::Status,
            Self::Login => State::Login,
        }
    }
}
impl Size for NextState {
    const SIZE: usize = i32::MIN_SIZE;
}
impl Readable for NextState {
    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        i32::read_var_from(reader)?
            .try_into()
            .map_err(|_| ReadError::InvalidEnumVariant)
    }
}
impl Writable for NextState {
    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {
        (*self as i32).write_var_to(writer)
    }
}
