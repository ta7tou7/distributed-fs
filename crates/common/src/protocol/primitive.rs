use crate::protocol::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::string::FromUtf8Error;

macro_rules! impl_serialize_for_numeric {
    ($($t:ty),*) => {
        $(
            impl Serialize for $t {
                fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
                    writer.write_all(&self.to_le_bytes())
                }
            }

            impl Deserialize for $t {
                fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
                    let mut buf = [0u8; std::mem::size_of::<$t>()];
                    reader.read_exact(&mut buf)?;
                    Ok(<$t>::from_le_bytes(buf))
                }
            }
        )*
    };
}
// Apply the macro to all target types at once
impl_serialize_for_numeric!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, usize
);

fn to_error(err: FromUtf8Error) -> Error {
    Error::new(ErrorKind::InvalidInput, err)
}

impl Serialize for bool {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[if *self { 1 } else { 0 }])
    }
}

impl Deserialize for bool {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        match buf[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid bool")),
        }
    }
}
impl Serialize for String {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let bytes = self.as_bytes();
        // sending 4 byte for length
        let len = bytes.len() as u32;
        len.serialize(writer)?;
        // sending the actual payload
        writer.write_all(&bytes)
    }
}

impl Deserialize for String {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let len = u32::deserialize(reader)?;
        let mut buffer = vec![0u8; len as usize];
        reader.read_exact(&mut buffer)?;
        String::from_utf8(buffer).map_err(to_error)
    }
}
