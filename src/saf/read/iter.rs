use std::io;

use super::{BinaryRead, PositionReader, ValueReader};

use crate::saf::{index, Site};

pub struct BinaryIterMut<'a, R>
where
    R: BinaryRead + ?Sized,
{
    inner: &'a mut R,
}

impl<'a, R> BinaryIterMut<'a, R>
where
    R: BinaryRead + ?Sized,
{
    pub fn new(inner: &'a mut R) -> Self {
        Self { inner }
    }
}

impl<'a, R> Iterator for BinaryIterMut<'a, R>
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

pub struct BinaryChunksMut<'a, R>
where
    R: BinaryRead + ?Sized,
{
    inner: &'a mut R,
    chunk_size: usize,
}

impl<'a, R> BinaryChunksMut<'a, R>
where
    R: BinaryRead + ?Sized,
{
    pub fn new(inner: &'a mut R, chunk_size: usize) -> Self {
        Self { inner, chunk_size }
    }
}

impl<'a, R> Iterator for BinaryChunksMut<'a, R>
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
    positions: BinaryIterMut<'a, PositionReader<R>>,
    values: BinaryChunksMut<'a, ValueReader<R>>,
    remaining: usize,
}

impl<'a, R> Sites<'a, R>
where
    R: io::BufRead,
{
    pub fn new(
        names: index::Names<'a>,
        positions: BinaryIterMut<'a, PositionReader<R>>,
        values: BinaryChunksMut<'a, ValueReader<R>>,
        n_sites: usize,
    ) -> Self {
        Self {
            names,
            positions,
            values,
            remaining: n_sites,
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
            (Some(n), Some(p), Some(v)) => {
                self.remaining -= 1;

                Some(Site::from_io(n, p, v))
            }
            _ => Some(Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "incompatible SAF file lengths",
            ))),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, R> ExactSizeIterator for Sites<'a, R>
where
    R: io::BufRead,
{
    fn len(&self) -> usize {
        self.remaining
    }
}
