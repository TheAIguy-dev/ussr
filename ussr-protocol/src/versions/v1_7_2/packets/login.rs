use super::*;

pub mod serverbound {
    use super::*;

    packet! {
        #[derive(Debug, Clone, PartialEq, Eq)]
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub EncryptionResponse {
            pub shared_secret: Vec<u8> = read_array::<u16, u8>, |writer, value| write_array::<u16, u8>(writer, value.as_slice()),
            pub verify_token: Vec<u8> = read_array::<u16, u8>, |writer, value| write_array::<u16, u8>(writer, value.as_slice()),
        }
        const ID = 0x01,
        const DIRECTION = Serverbound,
        const STATE = Login,
        const MIN_SIZE = 0,
        const MAX_SIZE = 0,
    }
}
