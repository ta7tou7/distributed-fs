use crate::protocol::{Deserialize, Serialize, Tagged, TaggedRef};
use std::io::{Error, ErrorKind, Read, Result, Write};
pub enum ChunkServerMessage {
    HeartBeat(Heartbeat),
    CheckPoint(CheckPoint),
}

#[derive(Debug)]
pub struct Heartbeat {}

impl Serialize for Heartbeat {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Tagged for Heartbeat {
    const TAG: u8 = 2;
}

#[derive(Debug)]
pub struct CheckPoint {}

impl Serialize for CheckPoint {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Tagged for CheckPoint {
    const TAG: u8 = 3;
}

impl Serialize for ChunkServerMessage {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            ChunkServerMessage::CheckPoint(msg) => TaggedRef(msg).serialize(writer),
            ChunkServerMessage::HeartBeat(msg) => TaggedRef(msg).serialize(writer),
        }
    }
}

impl Deserialize for ChunkServerMessage {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let code = u8::deserialize(reader)?;
        match code {
            2 => Ok(ChunkServerMessage::HeartBeat(Heartbeat {})),
            3 => Ok(ChunkServerMessage::CheckPoint(CheckPoint {})),
            other => Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid chunk server message code {other}"),
            )),
        }
    }
}
