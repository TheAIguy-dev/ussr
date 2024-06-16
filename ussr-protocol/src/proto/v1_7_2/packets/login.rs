use super::*;

pub mod serverbound {
    use super::*;

    packet! {
        #[derive(Debug, Clone, PartialEq, Eq, Component)]
        pub LoginStart {
            pub username: String,
        }
        const ID = 0x00,
        const DIRECTION = Serverbound,
        const STATE = Login,
        const MIN_SIZE = String::MIN_SIZE,
        const MAX_SIZE = String::MAX_SIZE,
    }

    packet! {
        #[derive(Debug, Clone, PartialEq, Eq, Component)]
        pub EncryptionResponse {
            pub shared_secret: Vec<u8> = read_array::<u16, u8>,  write_array::<u16, u8>,
            pub verify_token: Vec<u8> = read_array::<u16, u8>, write_array::<u16, u8>,
        }
        const ID = 0x01,
        const DIRECTION = Serverbound,
        const STATE = Login,
        const MIN_SIZE = (u16::SIZE + u16::MIN as usize * u8::SIZE) * 2,
        const MAX_SIZE = (u16::SIZE + u16::MAX as usize * u8::SIZE) * 2,
    }
}

pub mod clientbound {
    use super::*;

    packet! {
        #[derive(Debug, Clone, PartialEq, Eq, Component)]
        pub Disconnect {
            /// JSON
            pub reason: String,
        }
        const ID = 0x00,
        const DIRECTION = Clientbound,
        const STATE = Login,
        const MIN_SIZE = String::MIN_SIZE,
        const MAX_SIZE = String::MAX_SIZE,
    }

    packet! {
        #[derive(Debug, Clone, PartialEq, Eq, Component)]
        pub EncryptionRequest {
            pub server_id: String,
            pub public_key: Vec<u8> = read_array::<u16, u8>, write_array::<u16, u8>,
            pub verify_token: Vec<u8> = read_array::<u16, u8>, write_array::<u16, u8>,
        }
        const ID = 0x01,
        const DIRECTION = Clientbound,
        const STATE = Login,
        const MIN_SIZE = String::MIN_SIZE + (u16::SIZE + u16::MIN as usize * u8::SIZE) * 2,
        const MAX_SIZE = String::MAX_SIZE + (u16::SIZE + u16::MAX as usize * u8::SIZE) * 2,
    }

    packet! {
        #[derive(Debug, Clone, PartialEq, Eq, Component)]
        pub LoginSuccess {
            pub uuid: Uuid,
            pub username: String,
        }
        const ID = 0x02,
        const DIRECTION = Clientbound,
        const STATE = Login,
        const MIN_SIZE = Uuid::SIZE + String::MIN_SIZE,
        const MAX_SIZE = Uuid::SIZE + String::MAX_SIZE,
    }
}
