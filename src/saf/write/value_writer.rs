use std::{fs, io, path};

use byteorder::{LittleEndian, WriteBytesExt};

use noodles_bgzf as bgzf;

use super::{write_saf_magic, BinaryWrite};

pub struct ValueWriter<W>
where
    W: io::Write,
{
    inner: bgzf::Writer<W>,
}

impl ValueWriter<io::BufWriter<fs::File>> {
    pub fn from_path<P>(path: &P) -> io::Result<Self>
    where
        P: AsRef<path::Path>,
    {
        fs::File::create(path)
            .map(io::BufWriter::new)
            .map(Self::new)
    }
}

impl<W> ValueWriter<W>
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

impl<W> BinaryWrite for ValueWriter<W>
where
    W: io::Write,
{
    type Value = f32;

    fn write(&mut self, value: Self::Value) -> io::Result<()> {
        self.inner.write_f32::<LittleEndian>(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::saf::read::{BinaryRead, ValueReader};

    #[test]
    fn write_from_slice() -> io::Result<()> {
        let mut writer = ValueWriter::new(Vec::new());

        writer.write_header()?;

        let values = vec![0.0, 0.32, 0.89];

        writer.write_all(&values)?;

        let src = writer.finish()?;

        let mut reader = ValueReader::new(&src[..]);

        assert_eq!("safv3", reader.read_header()?);

        for value in values {
            assert_eq!(reader.read()?, value);
        }

        Ok(())
    }
}
