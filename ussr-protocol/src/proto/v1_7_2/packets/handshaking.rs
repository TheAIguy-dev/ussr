use super::*;

pub mod serverbound {
    use super::*;

    packet! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub Handshake {
            #[var]
            pub protocol_version: i32,
            pub server_address: String,
            pub server_port: u16,
            pub next_state: enums::NextState,
        }
        const ID = 0x00,
        const DIRECTION = Serverbound,
        const STATE = Handshaking,
        const CAN_CHANGE_STATE = true,
        const MIN_SIZE = i32::MIN_SIZE + String::MIN_SIZE + u16::SIZE + enums::NextState::SIZE,
        const MAX_SIZE = i32::MAX_SIZE + String::MAX_SIZE + u16::SIZE + enums::NextState::SIZE,
    }
}

pub mod clientbound {}
