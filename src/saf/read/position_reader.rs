use std::{fs::File, io, path::Path};

use byteorder::{LittleEndian, ReadBytesExt};

use flate2::bufread::MultiGzDecoder;

use super::{read_saf_magic, BinaryRead};

use crate::utils;

#[derive(Debug)]
pub struct PositionReader<R> {
    inner: MultiGzDecoder<R>,
}

impl<R> PositionReader<R>
where
    R: io::BufRead,
{
    pub fn inner(&self) -> &MultiGzDecoder<R> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut MultiGzDecoder<R> {
        &mut self.inner
    }

    pub fn into_inner(self) -> MultiGzDecoder<R> {
        self.inner
    }

    pub fn new(reader: R) -> Self {
        Self {
            inner: MultiGzDecoder::new(reader),
        }
    }

    pub fn read_header(&mut self) -> io::Result<String> {
        let magic = read_saf_magic(&mut self.inner)?;

        utils::parse_magic(&magic)
    }
}

impl PositionReader<io::BufReader<File>> {
    pub fn from_path<P>(path: &P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        File::open(path).map(io::BufReader::new).map(Self::new)
    }
}

impl<R> BinaryRead for PositionReader<R>
where
    R: io::BufRead,
{
    type Value = u32;

    fn read(&mut self) -> io::Result<Self::Value> {
        self.inner.read_u32::<LittleEndian>()
    }

    fn read_exact(&mut self, buf: &mut [Self::Value]) -> io::Result<()> {
        self.inner.read_u32_into::<LittleEndian>(buf)
    }
}
