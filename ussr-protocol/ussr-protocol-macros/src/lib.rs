/// Define a packet.
///
/// # Example
///
/// ```
/// packet! {
///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
///     pub TestPacket {
///         #[var]
///         field_one: i32,
///         pub field_two: String,
///     }
///     0x42, Clientbound, Handshaking,
///     MIN_SIZE = i32::MIN_SIZE + String::MIN_SIZE,
///     MAX_SIZE = i32::MAX_SIZE + String::MAX_SIZE,
/// }
/// ```
#[macro_export]
macro_rules! packet {
    {
        $(#[$ty_meta:meta])*
        $vis:vis $name:ident {
            $(
                $( $(@$tag:tt)? #[var] )? $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
        const $id_name:ident        = $id_value:literal,
        const $direction_name:ident = $direction_value:ident,
        const $state_name:ident     = $state_value:ident,
        const $min_size_name:ident  = $min_size_value:expr,
        const $max_size_name:ident  = $max_size_value:expr,
    } => {
        $(#[$ty_meta])*
        pub struct $name {
            $( $field_vis $field_name : $field_ty, )*
        }
        impl Packet for $name {
            const $id_name: u32                    = $id_value;
            const $direction_name: PacketDirection = $direction_value;
            const $state_name: State               = $state_value;
            const $min_size_name: usize            = $min_size_value;
            const $max_size_name: usize            = $max_size_value;

            fn read(reader: &mut impl Read) -> Result<Self, PacketReadError> {
                Ok(Self {
                    $(
                        $field_name: packet! {
                            @internal { $( $($tag)? + )? }
                            { <$field_ty>::read_from(reader)? }
                            { <$field_ty>::read_var_from(reader)? }
                        },
                    )*
                })
            }

            fn write(&self, writer: &mut impl Write) -> io::Result<()> {
                $(
                    packet! {
                        @internal { $( $($tag)? + )? }
                        { self.$field_name.write_to(writer)? }
                        { self.$field_name.write_var_to(writer)? }
                    }
                )*
                Ok(())
            }
        }
    };
    ( @internal {   } { $($then:tt)* } { $($else:tt)* } ) => { $($then)* };
    ( @internal { + } { $($then:tt)* } { $($else:tt)* } ) => { $($else)* };
}
