use std::{fs::File, io, path::Path};

use byteorder::{LittleEndian, ReadBytesExt};

use flate2::bufread::MultiGzDecoder;

use super::{read_magic, BinaryRead};

pub struct PositionReader<R> {
    inner: MultiGzDecoder<R>,
}

impl<R> PositionReader<R>
where
    R: io::BufRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            inner: MultiGzDecoder::new(reader),
        }
    }
}

impl PositionReader<io::BufReader<File>> {
    pub fn from_path<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut reader = File::open(path).map(io::BufReader::new).map(Self::new)?;

        read_magic(&mut reader.inner)?;

        Ok(reader)
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