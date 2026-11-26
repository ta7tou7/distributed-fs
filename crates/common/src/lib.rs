use crate::protocol::{Deserialize, Serialize};
use std::io::{Read, Write};

pub mod model;
pub mod protocol;
pub mod threadpool;

const MAX_FRAME_SIZE: u32 = 16 * 1024;

pub fn send_frame<T: Serialize, W: Write>(writer: &mut W, msg: &T) -> std::io::Result<()> {
    let mut payload = Vec::new();
    msg.serialize(&mut payload)?;
    let frame_length = u32::try_from(payload.len()).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Frame exceeds u32::Max")
    })?;
    if frame_length > MAX_FRAME_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Frame exceeds MAX_FRAME_SIZE",
        ));
    }
    frame_length.serialize(writer)?;
    writer.write_all(&payload)?;
    writer.flush()
}

pub fn receive_frame<T: Deserialize, R: Read>(reader: &mut R) -> std::io::Result<T> {
    let frame_length = u32::deserialize(reader)?;
    if frame_length > MAX_FRAME_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Incoming frame size {frame_length} exceeds MAX_FRAME_SIZE"),
        ));
    }
    let mut frame_buffer = vec![0u8; frame_length as usize];
    reader.read_exact(&mut frame_buffer)?;
    let mut cursor = std::io::Cursor::new(frame_buffer);
    let msg = T::deserialize(&mut cursor)?;
    if cursor.position() != frame_length as u64 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Trailing unread bytes inside frame",
        ));
    };
    Ok(msg)
}
