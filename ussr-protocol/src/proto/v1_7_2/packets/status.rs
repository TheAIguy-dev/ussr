use super::*;

pub mod serverbound {
    use super::*;

    packet! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub StatusRequest {}
        const ID = 0x00,
        const DIRECTION = Serverbound,
        const STATE = Status,
        const MIN_SIZE = 0,
        const MAX_SIZE = 0,
    }

    packet! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub PingRequest {
            pub payload: u64,
        }
        const ID = 0x01,
        const DIRECTION = Serverbound,
        const STATE = Status,
        const MIN_SIZE = u64::SIZE,
        const MAX_SIZE = u64::SIZE,
    }
}

pub mod clientbound {
    use super::*;

    packet! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub StatusResponse {
            /// JSON
            pub response: String,
        }
        const ID = 0x00,
        const DIRECTION = Clientbound,
        const STATE = Status,
        const MIN_SIZE = u64::SIZE,
        const MAX_SIZE = u64::SIZE,
    }

    packet! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub PingResponse {
            pub payload: u64,
        }
        const ID = 0x01,
        const DIRECTION = Clientbound,
        const STATE = Status,
        const MIN_SIZE = u64::SIZE,
        const MAX_SIZE = u64::SIZE,
    }
}
