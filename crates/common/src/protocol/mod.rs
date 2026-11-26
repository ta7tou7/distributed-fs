use std::io::{Read, Write};

pub mod chunkserver_master;
pub mod client_master;
pub mod primitive;

pub trait Tagged {
    const TAG: u8;
}
pub struct TaggedRef<'a, T>(pub &'a T);

pub trait Serialize {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

pub trait Deserialize: Sized {
    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self>;
}

impl<T: Serialize + Tagged> Serialize for TaggedRef<'_, T> {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[T::TAG])?;
        self.0.serialize(writer)
    }
}
