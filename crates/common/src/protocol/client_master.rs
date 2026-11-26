use crate::protocol::{Deserialize, Serialize, Tagged, TaggedRef};
use std::io::{BufReader, Read, Write};
use std::net::IpAddr;
#[derive(Debug)]
pub enum ClientMasterMessage {
    CreateFileRequest(CreateFileRequest),
    CreateFileResponse(CreateFileResponse),
}
impl Serialize for ClientMasterMessage {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            ClientMasterMessage::CreateFileRequest(req) => TaggedRef(req).serialize(writer),
            ClientMasterMessage::CreateFileResponse(req) => TaggedRef(req).serialize(writer),
        }
    }
}

impl Deserialize for ClientMasterMessage {
    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let tag = u8::deserialize(reader)?;
        match tag {
            CreateFileRequest::TAG => Ok(ClientMasterMessage::CreateFileRequest(
                CreateFileRequest::deserialize(reader)?,
            )),
            CreateFileResponse::TAG => Ok(ClientMasterMessage::CreateFileResponse(
                CreateFileResponse::deserialize(reader)?,
            )),
            unknown => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown protocol tag: {unknown:#04x}"),
            )),
        }
    }
}
#[derive(Debug)]
pub struct CreateFileRequest {
    pub path: String,
    pub replication: u8,
}

#[derive(Debug)]
pub struct CreateFileResponse {
    pub uid: u32,
    pub chunk_servers: Vec<(IpAddr, u16)>,
}

impl Serialize for CreateFileRequest {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        assert!(self.path.len() < u16::MAX as usize);
        self.path.serialize(writer)?;
        self.replication.serialize(writer)?;
        Ok(())
    }
}

impl Deserialize for CreateFileRequest {
    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buffer = BufReader::new(reader);
        let path = String::deserialize(&mut buffer)?;
        let replication = u8::deserialize(&mut buffer)?;
        Ok(CreateFileRequest { path, replication })
    }
}

impl Tagged for CreateFileRequest {
    const TAG: u8 = 0;
}

impl Serialize for CreateFileResponse {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.uid.serialize(writer)
    }
}

impl Deserialize for CreateFileResponse {
    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let uid = u32::deserialize(reader)?;
        Ok(CreateFileResponse {
            uid,
            chunk_servers: Vec::new(),
        })
    }
}

impl Tagged for CreateFileResponse {
    const TAG: u8 = 1;
}
