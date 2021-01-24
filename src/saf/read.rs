use std::io;

pub mod iter;
mod merged_reader;
mod position_reader;
mod reader;
mod value_reader;

pub use self::{
    merged_reader::MergedReader, position_reader::PositionReader, reader::Reader,
    value_reader::ValueReader,
};

use crate::saf::constants::MAGIC;

pub trait BinaryRead {
    type Value: Copy + Default;

    fn read(&mut self) -> io::Result<Self::Value>;

    fn read_exact(&mut self, buf: &mut [Self::Value]) -> io::Result<()>;

    fn chunks(&mut self, chunk_size: usize) -> iter::BinaryChunks<Self> {
        iter::BinaryChunks::new(self, chunk_size)
    }

    fn iter(&mut self) -> iter::BinaryIterator<Self> {
        iter::BinaryIterator::new(self)
    }
}

pub(crate) fn read_magic<R>(reader: &mut R) -> io::Result<([u8; 8])>
where
    R: io::Read,
{
    let mut magic = [0; 8];

    reader.read_exact(&mut magic)?;

    if magic == MAGIC {
        Ok(magic)
    } else {
        let msg = format!(
            "invalid magic number (expected {:x?}, found {:x?})",
            MAGIC, magic
        );

        Err(io::Error::new(io::ErrorKind::InvalidData, msg))
    }
}

pub(crate) fn parse_magic(magic: &[u8; 8]) -> io::Result<String> {
    match std::str::from_utf8(&magic.to_vec()) {
        Ok(s) => {
            let parsed = s.trim_matches(char::from(0));

            Ok(parsed.to_string())
        }
        Err(_) => {
            let msg = format!("unparseable magic number {:x?}", magic);

            Err(io::Error::new(io::ErrorKind::InvalidData, msg))
        }
    }
}
