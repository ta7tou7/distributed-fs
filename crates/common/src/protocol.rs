use crate::serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Read, Result, Write};
pub enum ChunkServerMessage {
    Join,
    HeartBeat,
}

impl Serialize for ChunkServerMessage {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            ChunkServerMessage::Join => 0u8.serialize(writer),
            ChunkServerMessage::HeartBeat => 1u8.serialize(writer),
        }
    }
}

impl Deserialize for ChunkServerMessage {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let code = u8::deserialize(reader)?;
        match code {
            0 => Ok(ChunkServerMessage::Join),
            1 => Ok(ChunkServerMessage::HeartBeat),
            other => Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid chunk server message code {other}"),
            )),
        }
    }
}
