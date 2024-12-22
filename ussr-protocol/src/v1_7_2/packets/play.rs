use super::*;

#[packets(Play)]
pub mod clientbound {
    use std::io::{self, Read, Write};

    #[cfg(feature = "async")]
    use futures_lite::{AsyncReadExt, AsyncWriteExt};
    #[cfg(feature = "async")]
    use ussr_buf::{AsyncDecodeExt, AsyncEncodeExt};
    use ussr_buf::{DecodeError, DecodeExt, EncodeExt};

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Decode, Encode)]
    #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
    pub struct KeepAlive {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct JoinGame {
        entity_id: i32,
        hardcore: bool,
        gamemode: enums::Gamemode,
        dimension: enums::Dimension,
        difficulty: enums::Difficulty,
        max_players: u8,
        level_type: String,
    }

    impl Decode for JoinGame {
        fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
            let entity_id: i32 = reader.decode()?;
            let gamemode: u8 = reader.decode()?;
            let hardcore: bool = gamemode & 0x8 != 0;
            let gamemode: enums::Gamemode = (gamemode & !0x8).try_into()?;

            Ok(JoinGame {
                entity_id,
                hardcore,
                gamemode,
                dimension: reader.decode()?,
                difficulty: reader.decode()?,
                max_players: reader.decode()?,
                level_type: reader.decode()?,
            })
        }
    }

    impl Encode for JoinGame {
        fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
            writer.encode(self.entity_id)?;
            writer.encode(self.gamemode as u8 | (self.hardcore as u8) << 3)?;
            writer.encode(self.dimension)?;
            writer.encode(self.difficulty)?;
            writer.encode(self.max_players)?;
            writer.encode(&self.level_type)
        }
    }

    #[cfg(feature = "async")]
    impl AsyncDecode for JoinGame {
        async fn decode(
            reader: &mut (impl AsyncReadExt + Unpin + Send),
        ) -> Result<Self, DecodeError> {
            let entity_id: i32 = reader.decode().await?;
            let gamemode: u8 = reader.decode().await?;
            let hardcore: bool = gamemode & 0x8 != 0;
            let gamemode: enums::Gamemode = (gamemode & !0x8).try_into()?;

            Ok(JoinGame {
                entity_id,
                hardcore,
                gamemode,
                dimension: reader.decode().await?,
                difficulty: reader.decode().await?,
                max_players: reader.decode().await?,
                level_type: reader.decode().await?,
            })
        }
    }

    #[cfg(feature = "async")]
    impl AsyncEncode for JoinGame {
        async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
            writer.encode(self.entity_id).await?;
            writer
                .encode(self.gamemode as u8 | (self.hardcore as u8) << 3)
                .await?;
            writer.encode(self.dimension).await?;
            writer.encode(self.difficulty).await?;
            writer.encode(self.max_players).await?;
            writer.encode(&self.level_type).await
        }
    }
}

#[packets(Play)]
pub mod serverbound {
    use super::*;

    // #[derive(Debug, Clone, Copy, PartialEq, Eq, Decode, Encode)]
    // #[cfg_attr(feature = "async", derive(AsyncDecode, AsyncEncode))]
}
