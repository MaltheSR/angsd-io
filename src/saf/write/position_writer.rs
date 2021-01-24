use std::io;

use byteorder::{LittleEndian, WriteBytesExt};

use noodles_bgzf as bgzf;

use super::{BinaryWrite, write_magic};

pub struct PositionWriter<W>
where
    W: io::Write
{
    inner: bgzf::Writer<W>
}

impl<W> PositionWriter<W>
where
    W: io::Write
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
        write_magic(&mut self.inner)
    }
}

impl<W> BinaryWrite for PositionWriter<W>
where
    W: io::Write
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

        let positions = vec![0, 32, 89];

        writer.write_all(&positions)?;

        let src = writer.finish()?;

        let mut reader = PositionReader::new(&src[..]);

        for pos in positions {
            assert_eq!(reader.read()?, pos);
        }

        Ok(())
    }
}