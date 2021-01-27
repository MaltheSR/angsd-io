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

use crate::{saf::constants::SAF_V3_MAGIC, utils};

pub trait BinaryRead {
    type Value: Copy + Default;

    fn read(&mut self) -> io::Result<Self::Value>;

    fn read_exact(&mut self, buf: &mut [Self::Value]) -> io::Result<()>;

    fn chunks_mut(&mut self, chunk_size: usize) -> iter::BinaryChunksMut<Self> {
        iter::BinaryChunksMut::new(self, chunk_size)
    }

    fn into_chunks(self, chunk_size: usize) -> iter::BinaryChunks<Self>
    where
        Self: Sized,
    {
        iter::BinaryChunks::new(self, chunk_size)
    }

    fn into_iter(self) -> iter::BinaryIter<Self>
    where
        Self: Sized,
    {
        iter::BinaryIter::new(self)
    }

    fn iter_mut(&mut self) -> iter::BinaryIterMut<Self> {
        iter::BinaryIterMut::new(self)
    }
}

pub(super) fn read_saf_magic<R>(reader: &mut R) -> io::Result<([u8; 8])>
where
    R: io::Read,
{
    utils::read_magic(reader, SAF_V3_MAGIC)
}
