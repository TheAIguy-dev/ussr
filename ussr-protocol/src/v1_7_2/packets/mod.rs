pub mod handshaking;
pub mod login;
pub mod play;
pub mod status;

#[cfg(feature = "async")]
use ussr_buf::{AsyncDecode, AsyncEncode};
use ussr_buf::{Decode, Encode};
use ussr_protocol_macros::packets;

use super::enums;
