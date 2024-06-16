pub mod handshaking;
pub mod login;
pub mod status;

use std::io::{self, Read, Write};

use bevy_ecs::component::Component;
use ussr_buf::{
    read::read_array, write::write_array, Readable, Size, VarReadable, VarSize, VarWritable,
    Writable,
};
use ussr_protocol_macros::packet;
use uuid::Uuid;

use crate::{
    Packet,
    PacketDirection::{self, *},
    PacketReadError,
};
// Using the latest enabled state
use crate::proto::enums::State::{self, *};
// But this version's enums and types
use super::{enums, types};

