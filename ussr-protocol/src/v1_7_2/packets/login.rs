use super::*;

#[packets(Login)]
pub mod clientbound {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct Disconnect {
        pub reason: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct EncryptionRequest {
        pub server_id: String,

        #[array(u16, u8)]
        pub public_key: Vec<u8>,

        #[array(u16, u8)]
        pub verify_token: Vec<u8>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct LoginSuccess {
        pub uuid: String,
        pub username: String,
    }
}

#[packets(Login)]
pub mod serverbound {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct LoginStart {
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct EncryptionResponse {
        #[array(u16, u8)]
        pub shared_secret: Vec<u8>,

        #[array(u16, u8)]
        pub verify_token: Vec<u8>,
    }
}
