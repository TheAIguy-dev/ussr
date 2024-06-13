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
            List { ty, .. } => format!("Vec<{}>", ty.to_string()),
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

pub fn read_integer_type(ty: &IntegerType) -> String {
    use IntegerType::*;
    match ty {
        VarInt | VarLong => "read_var_from",
        _ => "read_from",
    }
    .to_string()
}

pub fn write_integer_type(ty: &IntegerType) -> String {
    use IntegerType::*;
    match ty {
        VarInt | VarLong => "write_var_to",
        _ => "write_to",
    }
    .to_string()
}

pub fn read_base_type(ty: &BaseType) -> String {
    use BaseType::*;
    match ty {
        Integer(ty) => read_integer_type(ty),
        _ => "read_from".to_string(),
    }
}

pub fn read_type(ty: &Type) -> String {
    use Type::*;
    match ty {
        BaseType(ty) => read_base_type(ty),
        _ => "read_from".to_string(),
    }
}

pub fn is_copy(ty: &Type) -> bool {
    matches!(ty, Type::BaseType(ty) if !matches!(
        ty,
        BaseType::String { .. } | BaseType::List { .. } | BaseType::Nbt
    ))
}
