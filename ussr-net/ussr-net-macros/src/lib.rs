#[macro_export]
macro_rules! packet_decoder_map {
    {
        $commands:expr, $entity:expr, $state:expr, $packet_id:expr, $reader:expr,
        $(
            $s:pat => ( $($p:ty),* )
        ),* $(,)?
    } => {
        match $state {
            $(
                $s => match $packet_id {
                    $(
                        <$p>::ID => {
                            trace!(concat!("Reading ", stringify!($p)));
                            dispath_packet::<$p>($commands, $entity, $reader)?;
                            Ok(<$p>::CAN_CHANGE_STATE)
                        }
                    )*
                    _ => return Err(PacketReadError::UnknownPacketId { packet_id: $packet_id, state: $state }),
                }
            )*
        }
    };
}
