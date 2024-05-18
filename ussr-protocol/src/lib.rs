#![allow(unused)]
// Sample generated code from SpecMC

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
