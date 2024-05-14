use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    token::{self, Token},
    Ident, LitInt, Token,
};

#[proc_macro]
pub fn protocol_def(input: TokenStream) -> TokenStream {
    let packet: PacketDefinition = parse_macro_input!(input as PacketDefinition);
    println!("{}", packet.name);
    quote! {}.into()
}

#[derive(Debug)]
enum PacketDirection {
    Serverbound,
    Clientbound,
}
impl Parse for PacketDirection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<Ident>()?.to_string().as_str() {
            "serverbound" => Ok(PacketDirection::Serverbound),
            "clientbound" => Ok(PacketDirection::Clientbound),
            _ => Err(syn::Error::new(input.span(), "Invalid direction")),
        }
    }
}

enum PacketFieldType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    VarInt,
    VarLong,
    // TODO String,
    // TODO List,
    Nbt,
}
impl Parse for PacketFieldType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<Ident>()?.to_string().as_str() {
            "bool" => Ok(PacketFieldType::Bool),
            "u8" => Ok(PacketFieldType::U8),
            "u16" => Ok(PacketFieldType::U16),
            "u32" => Ok(PacketFieldType::U32),
            "u64" => Ok(PacketFieldType::U64),
            "i8" => Ok(PacketFieldType::I8),
            "i16" => Ok(PacketFieldType::I16),
            "i32" => Ok(PacketFieldType::I32),
            "i64" => Ok(PacketFieldType::I64),
            "f32" => Ok(PacketFieldType::F32),
            "f64" => Ok(PacketFieldType::F64),
            "VarInt" => Ok(PacketFieldType::VarInt),
            "VarLong" => Ok(PacketFieldType::VarLong),
            // TODO "String" => Ok(PacketFieldType::String),
            // TODO "List" => Ok(PacketFieldType::List),
            "Nbt" => Ok(PacketFieldType::Nbt),
            _ => Err(syn::Error::new(input.span(), "Invalid type")),
        }
    }
}

struct PacketField {
    ty: PacketFieldType,
    name: Ident,
    value: Option<Ident>,
}
impl Parse for PacketField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: PacketFieldType = input.parse()?;
        let name: Ident = input.parse()?;

        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let value: Ident = input.parse()?;
            Ok(PacketField {
                ty,
                name,
                value: Some(value),
            })
        } else {
            Ok(PacketField {
                ty,
                name,
                value: None,
            })
        }
    }
}

struct PacketDefinition {
    name: Ident,
    direction: PacketDirection,
    id: LitInt,
    fields: Vec<PacketField>,
}
impl Parse for PacketDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        assert_eq!(input.parse::<Ident>()?.to_string(), "packet");

        let name: Ident = input.parse::<Ident>()?;

        let packet_info: ParseBuffer;
        parenthesized!(packet_info in input);
        let direction: PacketDirection = packet_info.parse()?;
        packet_info.parse::<Token![,]>()?;
        let id: LitInt = packet_info.parse::<LitInt>()?;

        let packet_fields: ParseBuffer;
        braced!(packet_fields in input);
        let mut fields: Vec<PacketField> = Vec::new();
        while !packet_fields.is_empty() {
            fields.push(packet_fields.parse()?);
        }

        Ok(PacketDefinition {
            name,
            direction,
            id,
            fields,
        })
    }
}
