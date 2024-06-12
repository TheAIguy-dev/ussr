use specmc_protocol::{
    base::{BaseType, IntegerType},
    packets::Direction,
    types::Type,
};

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Type {
    fn to_string(&self) -> String {
        use Type::*;
        match self {
            BaseType(ty) => ty.to_string(),
            CustomType(ty) => ty.0.to_string(),
        }
    }
}

impl ToString for BaseType {
    fn to_string(&self) -> String {
        use BaseType::*;
        match self {
            Bool => "bool".to_string(),
            Integer(ty) => ty.to_string(),
            F32 => "f32".to_string(),
            F64 => "f64".to_string(),
            String { .. } => "String".to_string(),
            List { ty, .. } => ty.to_string(),
            Nbt => "Nbt".to_string(),
        }
    }
}

impl ToString for IntegerType {
    fn to_string(&self) -> String {
        use IntegerType::*;
        match self {
            I8 => "i8".to_string(),
            I16 => "i16".to_string(),
            I32 | VarInt => "i32".to_string(),
            I64 => "i64".to_string(),
            U8 => "u8".to_string(),
            U16 => "u16".to_string(),
            U32 | VarLong => "u32".to_string(),
            U64 => "u64".to_string(),
            // VarInt => "VarInt".to_string(),
            // VarLong => "VarLong".to_string(),
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        use Direction::*;
        match self {
            Serverbound => "Serverbound",
            Clientbound => "Clientbound",
        }
        .to_string()
    }
}
