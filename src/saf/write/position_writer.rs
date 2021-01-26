use std::{fs, io, path};

use byteorder::{LittleEndian, WriteBytesExt};

use noodles_bgzf as bgzf;

use super::{write_saf_magic, BinaryWrite};

pub struct PositionWriter<W>
where
    W: io::Write,
{
    inner: bgzf::Writer<W>,
}

impl PositionWriter<io::BufWriter<fs::File>> {
    pub fn from_path<P>(path: &P) -> io::Result<Self>
    where
        P: AsRef<path::Path>,
    {
        fs::File::create(path)
            .map(io::BufWriter::new)
            .map(Self::new)
    }
}

impl<W> PositionWriter<W>
where
    W: io::Write,
{
    pub fn finish(self) -> io::Result<W> {
        self.inner.finish()
    }

    pub fn new(writer: W) -> Self {
        Self {
            inner: bgzf::Writer::new(writer),
        }
    }

    pub fn write_header(&mut self) -> io::Result<()> {
        write_saf_magic(&mut self.inner)
    }
}

impl<W> BinaryWrite for PositionWriter<W>
where
    W: io::Write,
{
    type Value = u32;

    fn write(&mut self, value: Self::Value) -> io::Result<()> {
        self.inner.write_u32::<LittleEndian>(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::saf::read::{BinaryRead, PositionReader};

    #[test]
    fn write_from_slice() -> io::Result<()> {
        let mut writer = PositionWriter::new(Vec::new());

        writer.write_header()?;

        let positions = vec![0, 32, 89];

        writer.write_all(&positions)?;

        let src = writer.finish()?;

        let mut reader = PositionReader::new(&src[..]);

        assert_eq!("safv3", reader.read_header()?);

        for position in positions {
            assert_eq!(reader.read()?, position);
        }

        Ok(())
    }
}
