use super::*;

use crate::saf::{index, Site};

pub struct BinaryIterator<'a, R>
where
    R: BinaryRead + ?Sized,
{
    inner: &'a mut R,
}

impl<'a, R> BinaryIterator<'a, R>
where
    R: BinaryRead + ?Sized,
{
    pub fn new(inner: &'a mut R) -> Self {
        Self { inner }
    }
}

impl<'a, R> Iterator for BinaryIterator<'a, R>
where
    R: BinaryRead + ?Sized,
{
    type Item = io::Result<R::Value>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.read() {
            Ok(v) => Some(Ok(v)),
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub struct BinaryChunks<'a, R>
where
    R: BinaryRead + ?Sized,
{
    inner: &'a mut R,
    chunk_size: usize,
}

impl<'a, R> BinaryChunks<'a, R>
where
    R: BinaryRead + ?Sized,
{
    pub fn new(inner: &'a mut R, chunk_size: usize) -> Self {
        Self { inner, chunk_size }
    }
}

impl<'a, R> Iterator for BinaryChunks<'a, R>
where
    R: BinaryRead + ?Sized,
{
    type Item = io::Result<Vec<R::Value>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut values = Vec::new();
        values.resize(self.chunk_size, R::Value::default());

        match self.inner.read_exact(values.as_mut_slice()) {
            Ok(()) => Some(Ok(values)),
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub struct Sites<'a, R>
where
    R: io::BufRead,
{
    names: index::Names<'a>,
    positions: BinaryIterator<'a, PositionReader<R>>,
    values: BinaryChunks<'a, ValueReader<R>>,
}

impl<'a, R> Sites<'a, R>
where
    R: io::BufRead,
{
    pub fn new(
        names: index::Names<'a>,
        positions: BinaryIterator<'a, PositionReader<R>>,
        values: BinaryChunks<'a, ValueReader<R>>,
    ) -> Self {
        Self {
            names,
            positions,
            values,
        }
    }
}

impl<'a, R> Iterator for Sites<'a, R>
where
    R: io::BufRead,
{
    type Item = io::Result<Site<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.names.next(), self.positions.next(), self.values.next()) {
            (None, None, None) => None,
            (Some(n), Some(p), Some(v)) => Some(Site::from_io(n, p, v)),
            _ => Some(Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "incompatible SAF file lengths",
            ))),
        }
    }
}
