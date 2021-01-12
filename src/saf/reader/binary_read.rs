use std::io;

pub trait BinaryRead {
    type Value: Copy + Default;

    fn read(&mut self) -> io::Result<Self::Value>;

    fn read_exact(&mut self, buf: &mut [Self::Value]) -> io::Result<()>;

    fn chunks<'a>(&'a mut self, chunk_size: usize) -> BinaryChunks<'a, Self> {
        BinaryChunks::new(self, chunk_size)
    }

    fn iter<'a>(&'a mut self) -> BinaryIterator<'a, Self> {
        BinaryIterator::new(self)
    }
}

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
