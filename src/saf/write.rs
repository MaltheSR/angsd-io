use std::io;

use crate::{saf::constants::SAF_V3_MAGIC, utils};

mod position_writer;
mod value_writer;

pub use self::{position_writer::PositionWriter, value_writer::ValueWriter};

pub trait BinaryWrite {
    type Value: Copy + Default;

    fn write(&mut self, value: Self::Value) -> io::Result<()>;

    fn write_all<'a, I>(&mut self, values: I) -> io::Result<usize>
    where
        I: IntoIterator<Item = &'a Self::Value>,
        Self::Value: 'a,
    {
        let mut counter = 0;

        for value in values.into_iter() {
            self.write(*value)?;

            counter += 1;
        }

        Ok(counter)
    }
}

pub(self) fn write_saf_magic<W>(writer: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    utils::write_magic(writer, &SAF_V3_MAGIC)
}
