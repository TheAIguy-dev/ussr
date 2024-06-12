mod util;

use std::{env, fs, path::Path};

use specmc_base::{parse::Parse, tokenize};
use specmc_protocol::{
    base::IntegerType,
    enums::Enum,
    packets::Packet,
    spec::V1_7_2,
    types::{CustomType, Type},
    Protocol,
};

use util::ToString;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir: String = env::var("OUT_DIR").unwrap();
    let out_dir: &Path = Path::new(&out_dir);

    let protocol: Protocol = parse_spec(V1_7_2);

    let (enum_names, enums) = generate_enums(protocol.enums);
    fs::write(out_dir.join("enums.rs"), enums).unwrap();

    let (type_names, types) = generate_types(protocol.types);
    fs::write(out_dir.join("types.rs"), types).unwrap();

    let packets: String = generate_packets(protocol.packets, enum_names, type_names);
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

fn generate_enums(enums: Vec<Enum>) -> (Vec<String>, String) {
    let mut names: Vec<String> = vec![];
    let mut generated: String = "use std::io::{self, Read, Write};
                                use strum_macros::Display;
                                use ussr_buf::{ReadError, Readable, VarReadable, VarWritable, Writable};".to_string();

    for mut r#enum in enums {
        names.push(r#enum.name.to_string());

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
                generated += &format!("    {} = {},", variant.name, i);
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
                r#enum.ty.to_string(),
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
                "impl Readable for {0} {{
                    #[inline]
                    fn read_from(buf: &mut impl Read) -> Result<Self, ReadError> {{
                        {1}::{2}(buf)?.try_into()
                    }}
                }}",
                r#enum.name,
                r#enum.ty.to_string(),
                match r#enum.ty {
                    IntegerType::VarInt | IntegerType::VarLong => "read_var_from",
                    _ => "read_from",
                }
            );
        }

        // impl Writable for enum
        {
            generated += &format!(
                "impl Writable for {} {{
                    #[inline]
                    fn write_to(&self, buf: &mut impl Write) -> io::Result<()> {{
                        (*self as {}).{}(buf)
                    }}
                }}",
                r#enum.name,
                r#enum.ty.to_string(),
                match r#enum.ty {
                    IntegerType::VarInt | IntegerType::VarLong => "write_var_to",
                    _ => "write_to",
                }
            );
        }
    }

    (names, generated)
}

fn generate_types(types: Vec<CustomType>) -> (Vec<String>, String) {
    let mut names: Vec<String> = vec![];
    let mut generated: String = "".to_string();

    for ty in types {
        names.push(ty.name.0.clone());

        generated += &format!("pub struct {} {{", ty.name);

        for field in &ty.fields.0 {
            let mut ty: String = field.ty.to_string();
            if !field.conditions.is_empty() {
                ty = format!("Option<{}>", ty);
            }
            generated += &format!("pub {}: {},", field.name, ty);
        }

        generated += "}";
    }

    (names, generated)
}

fn generate_packets(
    packets: Vec<Packet>,
    enum_names: Vec<String>,
    type_names: Vec<String>,
) -> String {
    let mut generated: String = "".to_string();

    for packet in packets {
        generated += &format!("pub struct {} {{", packet.name);

        for field in &packet.fields.0 {
            let mut ty: String = match &field.ty {
                Type::BaseType(ty) => ty.to_string(),
                Type::CustomType(ty) => {
                    if enum_names.iter().any(|name| name == &ty.0) {
                        format!("super::enums::{ty}")
                    } else if type_names.iter().any(|name| name == &ty.0) {
                        format!("super::types::{ty}")
                    } else {
                        format!("{ty}")
                    }
                }
            };
            if !field.conditions.is_empty() {
                ty = format!("Option<{}>", ty);
            }
            generated += &format!("    pub {}: {},", field.name, ty);
        }

        generated += "}";

        // generated += &format!(
        //     "impl crate::Packet for {} {{
        //     const ID: u32 = {}
        //     const DIRECTION: crate::Direction = {}
        //     const STATE: crate::State = {}

        //     fn read(buf: &mut impl Read) -> Result<Self, PacketReadError> {{",
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
