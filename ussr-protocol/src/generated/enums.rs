use std::io::{self, Read, Write};
use strum_macros::Display;
use ussr_buf::{ReadError, Readable, VarReadable, VarWritable, Writable};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}
impl TryFrom<i32> for State {
    type Error = ReadError;
    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(State::Handshake),
            1 => Ok(State::Status),
            2 => Ok(State::Login),
            3 => Ok(State::Play),
            _ => Err(ReadError::InvalidEnumVariant),
        }
    }
}
impl Readable for State {
    #[inline]
    fn read_from(buf: &mut impl Read) -> Result<Self, ReadError> {
        i32::read_from(buf)?.try_into()
    }
}
impl Writable for State {
    #[inline]
    fn write_to(&self, buf: &mut impl Write) -> io::Result<()> {
        (*self as i32).write_to(buf)
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NextState {
    Status = 1,
    Login = 2,
}
impl TryFrom<i32> for NextState {
    type Error = ReadError;
    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            _ => Err(ReadError::InvalidEnumVariant),
        }
    }
}
impl Readable for NextState {
    #[inline]
    fn read_from(buf: &mut impl Read) -> Result<Self, ReadError> {
        i32::read_var_from(buf)?.try_into()
    }
}
impl Writable for NextState {
    #[inline]
    fn write_to(&self, buf: &mut impl Write) -> io::Result<()> {
        (*self as i32).write_var_to(buf)
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mouse {
    LeftClick = 0,
    RightClick = 1,
}
impl TryFrom<i8> for Mouse {
    type Error = ReadError;
    #[inline]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mouse::LeftClick),
            1 => Ok(Mouse::RightClick),
            _ => Err(ReadError::InvalidEnumVariant),
        }
    }
}
impl Readable for Mouse {
    #[inline]
    fn read_from(buf: &mut impl Read) -> Result<Self, ReadError> {
        i8::read_from(buf)?.try_into()
    }
}
impl Writable for Mouse {
    #[inline]
    fn write_to(&self, buf: &mut impl Write) -> io::Result<()> {
        (*self as i8).write_to(buf)
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    StartDigging = 0,
    CancelDigging = 1,
    FinishDigging = 2,
    DropItemStack = 3,
    DropItem = 4,
    ShootArrowOrFinishEating = 5,
}
impl TryFrom<i8> for Status {
    type Error = ReadError;
    #[inline]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::StartDigging),
            1 => Ok(Status::CancelDigging),
            2 => Ok(Status::FinishDigging),
            3 => Ok(Status::DropItemStack),
            4 => Ok(Status::DropItem),
            5 => Ok(Status::ShootArrowOrFinishEating),
            _ => Err(ReadError::InvalidEnumVariant),
        }
    }
}
impl Readable for Status {
    #[inline]
    fn read_from(buf: &mut impl Read) -> Result<Self, ReadError> {
        i8::read_from(buf)?.try_into()
    }
}
impl Writable for Status {
    #[inline]
    fn write_to(&self, buf: &mut impl Write) -> io::Result<()> {
        (*self as i8).write_to(buf)
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    Down = 0,
    Up = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}
impl TryFrom<i8> for Face {
    type Error = ReadError;
    #[inline]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Face::Down),
            1 => Ok(Face::Up),
            2 => Ok(Face::North),
            3 => Ok(Face::South),
            4 => Ok(Face::West),
            5 => Ok(Face::East),
            _ => Err(ReadError::InvalidEnumVariant),
        }
    }
}
impl Readable for Face {
    #[inline]
    fn read_from(buf: &mut impl Read) -> Result<Self, ReadError> {
        i8::read_from(buf)?.try_into()
    }
}
impl Writable for Face {
    #[inline]
    fn write_to(&self, buf: &mut impl Write) -> io::Result<()> {
        (*self as i8).write_to(buf)
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Animation {
    None = 0,
    SwingArm = 1,
    TakeDamage = 2,
    LeaveBed = 3,
    EatFood = 4,
    CriticalEffect = 5,
    MagicCriticalEffect = 6,
    Unknown = 102,
    Crouch = 104,
    Uncrouch = 105,
}
impl TryFrom<i8> for Animation {
    type Error = ReadError;
    #[inline]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Animation::None),
            1 => Ok(Animation::SwingArm),
            2 => Ok(Animation::TakeDamage),
            3 => Ok(Animation::LeaveBed),
            4 => Ok(Animation::EatFood),
            5 => Ok(Animation::CriticalEffect),
            6 => Ok(Animation::MagicCriticalEffect),
            102 => Ok(Animation::Unknown),
            104 => Ok(Animation::Crouch),
            105 => Ok(Animation::Uncrouch),
            _ => Err(ReadError::InvalidEnumVariant),
        }
    }
}
impl Readable for Animation {
    #[inline]
    fn read_from(buf: &mut impl Read) -> Result<Self, ReadError> {
        i8::read_from(buf)?.try_into()
    }
}
impl Writable for Animation {
    #[inline]
    fn write_to(&self, buf: &mut impl Write) -> io::Result<()> {
        (*self as i8).write_to(buf)
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityAction {
    Crouch = 1,
    Uncrouch = 2,
    LeaveBed = 3,
    StartSprinting = 4,
    StopSprinting = 5,
}
impl TryFrom<i8> for EntityAction {
    type Error = ReadError;
    #[inline]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EntityAction::Crouch),
            2 => Ok(EntityAction::Uncrouch),
            3 => Ok(EntityAction::LeaveBed),
            4 => Ok(EntityAction::StartSprinting),
            5 => Ok(EntityAction::StopSprinting),
            _ => Err(ReadError::InvalidEnumVariant),
        }
    }
}
impl Readable for EntityAction {
    #[inline]
    fn read_from(buf: &mut impl Read) -> Result<Self, ReadError> {
        i8::read_from(buf)?.try_into()
    }
}
impl Writable for EntityAction {
    #[inline]
    fn write_to(&self, buf: &mut impl Write) -> io::Result<()> {
        (*self as i8).write_to(buf)
    }
}
