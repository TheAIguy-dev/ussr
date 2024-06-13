mod util;

use std::{collections::HashSet, env, fs, path::Path};

use specmc_base::{parse::Parse, tokenize};
use specmc_protocol::{
    base::{BaseType, Value},
    enums::Enum,
    packets::Packet,
    spec::V1_7_2,
    types::{CustomType, Type},
    Protocol,
};

use util::{is_copy, read_integer_type, write_integer_type, ToString};

// TODO
// - delete packet gen code
// - make type gen code work:
//   - make it create flags

fn main() {
    println!("cargo::rerun-if-changed=build/mod.rs");

    let out_dir: String = env::var("OUT_DIR").unwrap();
    let out_dir: &Path = Path::new(&out_dir);

    let protocol: Protocol = parse_spec(V1_7_2);

    let enums = generate_enums(protocol.enums);
    fs::write(out_dir.join("enums.rs"), enums).unwrap();

    let types = generate_types(protocol.types);
    fs::write(out_dir.join("types.rs"), types).unwrap();

    let packets: String = generate_packets(protocol.packets);
    fs::write(out_dir.join("packets.rs"), packets).unwrap();
}

fn parse_spec(spec: &str) -> Protocol {
    let mut protocol: String = "".to_string();
    for line in spec.split('\n') {
        if let Some(index) = line.find("//") {
            protocol += &line[..index];
        } else {
            protocol += line;
        }
        protocol += "\n";
    }
    Protocol::parse(&mut tokenize!(&protocol)).expect("Failed to parse protocol")
}

fn generate_enums(enums: Vec<Enum>) -> String {
    let mut generated: String = "use std::io::{self, Read, Write};
                                use strum_macros::Display;
                                use ussr_buf::{ReadError, Readable, VarReadable, VarWritable, Writable};".to_string();

    for mut r#enum in enums {
        // enum
        {
            generated += &format!(
                "#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
                pub enum {} {{",
                r#enum.name
            );

            let mut i: isize = 0;
            for variant in &mut r#enum.variants {
                if let Some(value) = variant.value {
                    i = value;
                } else {
                    variant.value = Some(i);
                }
                generated += &format!("{} = {},", variant.name, i);
                i += 1;
            }

            generated += "}";
        }

        // impl TryFrom for enum
        {
            generated += &format!(
                "impl TryFrom<{1}> for {0} {{
                    type Error = ReadError;
                    #[inline]
                    fn try_from(value: {1}) -> Result<Self, Self::Error> {{
                        match value {{",
                r#enum.name,
                r#enum.ty.to_string()
            );

            for variant in &r#enum.variants {
                generated += &format!(
                    "{} => Ok({}::{}),",
                    variant.value.unwrap(),
                    r#enum.name,
                    variant.name
                );
            }
            generated += "_ => Err(ReadError::InvalidEnumVariant),";

            generated += "}}}";
        }

        // impl Readable for enum
        {
            generated += &format!(
                "impl Readable for {} {{
                    #[inline]
                    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {{
                        <{}>::{}(reader)?.try_into()
                    }}
                }}",
                r#enum.name,
                r#enum.ty.to_string(),
                read_integer_type(&r#enum.ty)
            );
        }

        // impl Writable for enum
        {
            generated += &format!(
                "impl Writable for {} {{
                    #[inline]
                    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {{
                        (*self as {}).{}(writer)
                    }}
                }}",
                r#enum.name,
                r#enum.ty.to_string(),
                write_integer_type(&r#enum.ty)
            );
        }
    }

    generated
}

fn generate_types(types: Vec<CustomType>) -> String {
    fn to_string(ty: &Type) -> String {
        use Type::*;
        match ty {
            BaseType(ty) => ty.to_string(),
            CustomType(ty) => format!("enums::{}", ty.0),
        }
    }

    let mut generated: String = "use std::io::{self, Read, Write};
                                use ussr_buf::{ReadError, Readable, VarReadable, VarWritable, Writable};
                                use ussr_nbt::Nbt;
                                use super::enums;".to_string();

    for ty in types {
        // type
        {
            generated += &format!("pub struct {} {{", ty.name);

            for field in &ty.fields.0 {
                let mut ty: String = to_string(&field.ty);
                if !field.conditions.is_empty() {
                    ty = format!("Option<{}>", ty);
                }
                generated += &format!("pub {}: {},", field.name, ty);
            }

            generated += "}";
        }

        // impl Readable for type
        {
            generated += &format!(
                "impl Readable for {} {{
                    #[inline]
                    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {{",
                ty.name
            );

            for field in ty.fields.0.iter() {
                if !field.conditions.is_empty() {
                    generated += &format!("let mut _{} = None;", field.name);
                } else {
                    generated += &format!(
                        "let {} = <{}>::read_from(reader)?;",
                        field.name,
                        to_string(&field.ty)
                    );
                }
            }

            let mut conditions: HashSet<String> = HashSet::new();
            let mut scopes: Vec<Vec<String>> = vec![];
            let mut current_scope: Vec<String> = vec![];
            for field in &ty.fields.0 {
                if field.conditions.is_empty() {
                    continue;
                }
                if field.conditions.difference(&conditions).count() > 0 {
                    scopes.push(current_scope);
                    current_scope = vec![field.name.to_string()];
                    generated += &format!(
                        "if {} {{",
                        field
                            .conditions
                            .difference(&conditions)
                            .cloned()
                            .collect::<Vec<String>>()
                            .join(" && ")
                    );
                    conditions.extend(field.conditions.clone());
                } else if field.conditions == conditions {
                    current_scope.push(field.name.to_string());
                } else if field.conditions.is_subset(&conditions) {
                    for name in current_scope {
                        generated += &format!("_{0} = Some({0});", name);
                    }
                    generated += "}";
                    current_scope = scopes.pop().unwrap();
                    current_scope.push(field.name.to_string());
                    conditions = field.conditions.clone();
                }
                generated += &format!(
                    "let {} = <{}>::read_from(reader)?;",
                    field.name,
                    to_string(&field.ty)
                );
            }
            scopes.push(current_scope);
            for (i, scope) in scopes.into_iter().enumerate().rev() {
                for name in scope {
                    generated += &format!("_{0} = Some({0});", name);
                }
                if i != 0 {
                    generated += "}";
                }
            }

            generated += &format!("Ok({} {{", ty.name);
            for field in &ty.fields.0 {
                if field.conditions.is_empty() {
                    generated += &format!("{},", field.name);
                } else {
                    generated += &format!("{0}: _{0},", field.name);
                }
            }
            generated += "})}}";
        }

        // impl Writable for type
        {
            generated += &format!(
                "impl Writable for {} {{
                    #[inline]
                    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {{",
                ty.name
            );

            for field in ty.fields.0.iter() {
                if field.conditions.is_empty() {
                    if is_copy(&field.ty) {
                        generated += &format!("let {0} = self.{0};", field.name);
                    } else {
                        generated += &format!("let {0} = &self.{0};", field.name);
                    }
                } else {
                    generated += &format!("let {0} = &self.{0};", field.name);
                }
            }

            let mut conditions: HashSet<String> = HashSet::new();
            let mut scopes: Vec<Vec<String>> = vec![];
            let mut current_scope: Vec<String> = vec![];
            for field in &ty.fields.0 {
                if field.conditions.is_empty() {
                    generated += &format!("{}.write_to(writer)?;", field.name);
                    continue;
                }
                if field.conditions.difference(&conditions).count() > 0 {
                    scopes.push(current_scope);
                    current_scope = vec![field.name.to_string()];
                    generated += &format!(
                        "if {} {{",
                        field
                            .conditions
                            .difference(&conditions)
                            .cloned()
                            .collect::<Vec<String>>()
                            .join(" && ")
                    );
                    conditions.extend(field.conditions.clone());
                } else if field.conditions == conditions {
                    current_scope.push(field.name.to_string());
                } else if field.conditions.is_subset(&conditions) {
                    for name in current_scope {
                        generated += &format!("{}.write_to(writer)?;", name);
                    }
                    generated += "}";
                    current_scope = scopes.pop().unwrap();
                    current_scope.push(field.name.to_string());
                    conditions = field.conditions.clone();
                }
                if is_copy(&field.ty) {
                    generated += &format!("let {0} = {0}.unwrap();", field.name);
                } else {
                    generated += &format!("let {0} = {0}.as_ref().unwrap();", field.name);
                }
            }
            scopes.push(current_scope);
            for (i, scope) in scopes.into_iter().enumerate().rev() {
                for name in scope {
                    generated += &format!("{}.write_to(writer)?;", name);
                }
                if i != 0 {
                    generated += "}";
                }
            }

            generated += "Ok(())}}";
        }
    }

    generated
}

fn generate_packets(packets: Vec<Packet>) -> String {
    fn to_string(ty: &Type) -> String {
        use Type::*;
        match ty {
            BaseType(ty) => ty.to_string(),
            CustomType(ty) => format!("super::{}", ty.0),
        }
    }

    let mut generated: String = "use std::io::{self, Read, Write};
                                use ussr_buf::{ReadError, Readable, VarReadable, VarWritable, Writable};
                                use ussr_nbt::Nbt;
                                use crate::{Direction, PacketReadError, State};".to_string();

    for packet in packets {
        generated += &format!("pub struct {} {{", packet.name);

        for field in &packet.fields.0 {
            if matches!(field.value, Some(Value::Length(_))) {
                continue;
            }
            let mut ty: String = to_string(&field.ty);
            if !field.conditions.is_empty() {
                ty = format!("Option<{}>", ty);
            }
            generated += &format!("pub {}: {},", field.name, ty);
        }

        generated += "}";

        generated += &format!(
            "impl crate::Packet for {} {{
            const ID: u32 = {};
            const DIRECTION: Direction = Direction::{};
            const STATE: State = State::{};",
            packet.name,
            packet.id,
            packet.direction.to_string(),
            packet.state
        );

        // fn read
        {
            generated += "fn read(reader: &mut impl Read) -> Result<Self, PacketReadError> {";

            for field in packet.fields.0.iter() {
                if !field.conditions.is_empty() {
                    generated += &format!("let mut _{} = None;", field.name);
                } else {
                    generated += &format!(
                        "let {} = <{}>::read_from(reader)?;",
                        field.name,
                        to_string(&field.ty)
                    );
                }
            }

            let mut conditions: HashSet<String> = HashSet::new();
            let mut scopes: Vec<Vec<String>> = vec![];
            let mut current_scope: Vec<String> = vec![];
            for field in &packet.fields.0 {
                if field.conditions.is_empty() {
                    continue;
                }
                if field.conditions.difference(&conditions).count() > 0 {
                    scopes.push(current_scope);
                    current_scope = vec![field.name.to_string()];
                    generated += &format!(
                        "if {} {{",
                        field
                            .conditions
                            .difference(&conditions)
                            .cloned()
                            .collect::<Vec<String>>()
                            .join(" && ")
                    );
                    conditions.extend(field.conditions.clone());
                } else if field.conditions == conditions {
                    current_scope.push(field.name.to_string());
                } else if field.conditions.is_subset(&conditions) {
                    for name in current_scope {
                        generated += &format!("_{0} = Some({0});", name);
                    }
                    generated += "}";
                    current_scope = scopes.pop().unwrap();
                    current_scope.push(field.name.to_string());
                    conditions = field.conditions.clone();
                }
                generated += &format!(
                    "let {} = <{}>::read_from(reader)?;",
                    field.name,
                    to_string(&field.ty)
                );
            }
            scopes.push(current_scope);
            for (i, scope) in scopes.into_iter().enumerate().rev() {
                for name in scope {
                    generated += &format!("_{0} = Some({0});", name);
                }
                if i != 0 {
                    generated += "}";
                }
            }

            generated += &format!("Ok({} {{", packet.name);
            for field in &packet.fields.0 {
                if matches!(field.value, Some(Value::Length(_))) {
                    continue;
                }
                if field.conditions.is_empty() {
                    generated += &format!("{},", field.name);
                } else {
                    generated += &format!("{0}: _{0},", field.name);
                }
            }
            generated += "})}";
        }

        // fn write
        {
            generated += "fn write(&self, writer: &mut impl Write) -> io::Result<()> {";

            for field in packet.fields.0.iter() {
                if matches!(field.value, Some(Value::Length(_))) {
                    continue;
                }
                if field.conditions.is_empty() {
                    if is_copy(&field.ty) {
                        generated += &format!("let {0} = self.{0};", field.name);
                    } else {
                        generated += &format!("let {0} = &self.{0};", field.name);
                    }
                } else {
                    generated += &format!("let {0} = &self.{0};", field.name);
                }
            }

            let mut conditions: HashSet<String> = HashSet::new();
            let mut scopes: Vec<Vec<String>> = vec![];
            let mut current_scope: Vec<String> = vec![];
            for field in &packet.fields.0 {
                if field.conditions.is_empty() {
                    if !matches!(field.value, Some(Value::Length(_))) {
                        generated += &format!("{}.write_to(writer)?;", field.name);
                    }
                    continue;
                }
                if field.conditions.difference(&conditions).count() > 0 {
                    scopes.push(current_scope);
                    current_scope = vec![field.name.to_string()];
                    generated += &format!(
                        "if {} {{",
                        field
                            .conditions
                            .difference(&conditions)
                            .cloned()
                            .collect::<Vec<String>>()
                            .join(" && ")
                    );
                    conditions.extend(field.conditions.clone());
                } else if field.conditions == conditions {
                    current_scope.push(field.name.to_string());
                } else if field.conditions.is_subset(&conditions) {
                    for name in current_scope {
                        generated += &format!("{}.write_to(writer)?;", name);
                    }
                    generated += "}";
                    current_scope = scopes.pop().unwrap();
                    current_scope.push(field.name.to_string());
                    conditions = field.conditions.clone();
                }
                if is_copy(&field.ty) {
                    generated += &format!("let {0} = {0}.unwrap();", field.name);
                } else {
                    generated += &format!("let {0} = {0}.as_ref().unwrap();", field.name);
                }
            }
            scopes.push(current_scope);
            for (i, scope) in scopes.into_iter().enumerate().rev() {
                for name in scope {
                    generated += &format!("{}.write_to(writer)?;", name);
                }
                if i != 0 {
                    generated += "}";
                }
            }

            generated += "Ok(())}}";
        }

        //     packet.name,
        //     packet.id,
        //     packet.direction.to_string(),
        //     packet.state
        // );

        // for field in packet.fields.0 {
        //     if let
        // }
    }

    generated
}
