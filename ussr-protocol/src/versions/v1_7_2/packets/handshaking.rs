use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: enums::NextState,
}
impl Packet for Handshake {
    const ID: u32 = 0x00;
    const DIRECTION: PacketDirection = Serverbound;
    const STATE: State = Handshaking;
    const MIN_SIZE: usize = (i32::MIN_SIZE + String::MIN_SIZE + u16::SIZE + enums::NextState::SIZE);
    const MAX_SIZE: usize = (i32::MAX_SIZE + String::MAX_SIZE + u16::SIZE + enums::NextState::SIZE);
    fn read(reader: &mut impl Read) -> Result<Self, PacketReadError> {
        Ok(Self {
            protocol_version: <i32>::read_var_from(reader)?,
            server_address: <String>::read_from(reader)?,
            server_port: <u16>::read_from(reader)?,
            next_state: <enums::NextState>::read_from(reader)?,
        })
    }
    fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        packet! {
            @internal {
                +
            }{
                self.protocol_version.write_to(writer)?
            }{
                self.protocol_version.write_var_to(writer)?
            }
        }
        packet! {
            @internal{}
            {
                self.server_address.write_to(writer)?
            }{
                self.server_address.write_var_to(writer)?
            }
        }
        packet! {
            @internal{}
            {
                self.server_port.write_to(writer)?
            }{
                self.server_port.write_var_to(writer)?
            }
        }
        packet! {
            @internal{}
            {
                self.next_state.write_to(writer)?
            }{
                self.next_state.write_var_to(writer)?
            }
        }
        Ok(())
    }
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Handshake {
//     pub protocol_version: i32,
//     pub server_address: String,
//     pub server_port: u16,
//     pub next_state: enums::NextState,
// }
// impl Packet for Handshake {
//     const ID: u32 = 0x00;
//     const DIRECTION: PacketDirection = Serverbound;
//     const STATE: State = Handshaking;
//     const MIN_SIZE: usize = i32::MIN_SIZE + String::MIN_SIZE + u16::SIZE + enums::NextState::SIZE;
//     const MAX_SIZE: usize = i32::MAX_SIZE + String::MAX_SIZE + u16::SIZE + enums::NextState::SIZE;

//     fn read(reader: &mut impl Read) -> Result<Self, PacketReadError> {
//         Ok(Self {
//             protocol_version: i32::read_var_from(reader)?,
//             server_address: String::read_from(reader)?,
//             server_port: u16::read_from(reader)?,
//             next_state: enums::NextState::read_from(reader)?,
//         })
//     }

//     fn write(&self, writer: &mut impl Write) -> io::Result<()> {
//         self.protocol_version.write_var_to(writer)?;
//         self.server_address.write_to(writer)?;
//         self.server_port.write_to(writer)?;
//         self.next_state.write_to(writer)?;
//         Ok(())
//     }
// }
