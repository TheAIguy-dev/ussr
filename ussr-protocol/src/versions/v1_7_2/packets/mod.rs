pub mod handshaking;

use std::io::{self, Read, Write};

use ussr_buf::{Readable, Size, VarReadable, VarSize, VarWritable, Writable};
use ussr_protocol_macros::packet;

use crate::{
    Packet,
    PacketDirection::{self, *},
    PacketReadError,
};
// Using the latest enabled state
use super::super::enums::State::{self, *};
// But this version's enums and types
use super::{enums, types};
