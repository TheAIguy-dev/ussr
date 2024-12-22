mod common;
mod decode;
mod encode;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use common::wrap_result;
use decode::try_derive_decode;
use encode::try_derive_encode;

#[proc_macro_derive(Decode, attributes(var, array, with, ignore))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    wrap_result(
        try_derive_decode(&input, false),
        &input,
        "decode::Decode",
        "fn decode(reader: &mut impl std::io::Read) -> std::result::Result<Self, ussr_buf::DecodeError>",
    )
    .into()
}

#[proc_macro_derive(Encode, attributes(var, array, with, ignore))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    wrap_result(
        try_derive_encode(&input, false),
        &input,
        "encode::Encode",
        "fn encode(&self, writer: &mut impl std::io::Write) -> std::io::Result<()>",
    )
    .into()
}

#[proc_macro_derive(AsyncDecode, attributes(var, array, with, ignore))]
pub fn derive_async_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    wrap_result(
        try_derive_decode(&input, true),
        &input,
        "async_decode::Decode",
        "async fn decode(reader: &mut (impl futures_lite::AsyncReadExt + std::marker::Unpin + std::marker::Send)) -> std::result::Result<Self, ussr_buf::DecodeError>",
    )
    .into()
}

#[proc_macro_derive(AsyncEncode, attributes(var, array, with, ignore))]
pub fn derive_async_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    wrap_result(
        try_derive_encode(&input, true),
        &input,
        "async_encode::Encode",
        "async fn encode(&self, writer: &mut (impl futures_lite::AsyncWriteExt + std::marker::Unpin + std::marker::Send)) -> std::io::Result<()>",
    )
    .into()
}
