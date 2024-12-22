use super::*;

#[packets(Handshake)]
pub mod clientbound {}

#[packets(Handshake)]
pub mod serverbound {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct Handshake {
        #[var]
        pub protocol_version: i32,
        pub server_address: String,
        pub server_port: u16,
        pub next_state: enums::NextState,
    }
}
