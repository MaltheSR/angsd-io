use std::io;

use crate::saf::constants::MAGIC;

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

pub(crate) fn write_magic<W>(writer: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writer.write_all(&MAGIC)
}
