use super::*;

#[packets(Status)]
pub mod clientbound {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct StatusResponse {
        /// JSON
        pub response: String,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct PingResponse {
        pub payload: u64,
    }
}

#[packets(Status)]
pub mod serverbound {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct StatusRequest {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct PingRequest {
        pub payload: u64,
    }
}
