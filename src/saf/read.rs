use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};

#[cfg(feature = "ndarray")]
use ndarray::prelude::*;

use crate::saf::{constants::*, index};

pub mod iter;
mod merge;
mod position_reader;
mod value_reader;

pub use self::{merge::MergedReader, position_reader::PositionReader, value_reader::ValueReader};

pub struct Reader<R> {
    index: index::Index,
    position_reader: PositionReader<R>,
    value_reader: ValueReader<R>,
}

impl<R> Reader<R> {
    pub fn index(&self) -> &index::Index {
        &self.index
    }

    pub fn index_mut(&mut self) -> &mut index::Index {
        &mut self.index
    }

    pub fn new(
        index: index::Index,
        position_reader: PositionReader<R>,
        value_reader: ValueReader<R>,
    ) -> Self {
        Self {
            index,
            position_reader,
            value_reader,
        }
    }

    pub fn position_reader_mut(&mut self) -> &mut PositionReader<R> {
        &mut self.position_reader
    }

    pub fn value_reader_mut(&mut self) -> &mut ValueReader<R> {
        &mut self.value_reader
    }
}

impl<R> Reader<R>
where
    R: io::BufRead,
{
    #[cfg(feature = "ndarray")]
    pub fn read_values_to_array(mut self) -> io::Result<Array2<f32>> {
        let n_sites = self.index.n_sites();
        let n_alleles = self.index.n_alleles();

        let mut values = Vec::new();
        values.resize(n_sites * n_alleles, 0.0);

        self.value_reader
            .read_exact(values.as_mut_slice())
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "SAF value dimensions do not match metadata",
                )
            })?;

        let array = Array::from_shape_vec((n_sites, n_alleles), values).unwrap();

        Ok(array)
    }

    pub fn sites(&mut self) -> iter::Sites<R> {
        iter::Sites::new(
            self.index.iter_names(),
            self.position_reader.iter(),
            self.value_reader.chunks(self.index.n_alleles()),
        )
    }

    pub fn read_header(&mut self) -> io::Result<String> {
        // Ok to unwrap here, magic is checked in index construction
        let index_header = parse_magic(&self.index.magic())?;
        let position_header = self.position_reader.read_header()?;
        let value_header = self.value_reader.read_header()?;

        assert!(index_header == position_header && index_header == value_header);

        Ok(index_header)

    }
}

impl Reader<io::BufReader<File>> {
    pub fn from_paths<P>(index_path: P, position_path: P, value_path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let index = File::open(index_path).map(|x| index::Reader::new(x).read_index())??;

        let position_reader = PositionReader::from_path(position_path)?;

        let value_reader = ValueReader::from_path(value_path)?;

        Ok(Self::new(index, position_reader, value_reader))
    }

    pub fn from_member_path<P>(path: P) -> io::Result<Self>
    where
        P: Into<PathBuf>,
    {
        let prefix = find_prefix(path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "input is not a SAF member path",
            )
        })?;

        Self::from_prefix(prefix)
    }

    pub fn from_prefix<P>(prefix: P) -> io::Result<Self>
    where
        P: Into<PathBuf>,
    {
        let prefix = prefix.into();

        let mut index_path = prefix.clone();
        index_path.set_extension(INDEX_EXT);

        let mut value_path = prefix.clone();
        value_path.set_extension(VALUE_EXT);

        let mut position_path = prefix;
        position_path.set_extension(POSITION_EXT);

        Self::from_paths(index_path, position_path, value_path)
    }
}

pub trait BinaryRead {
    type Value: Copy + Default;

    fn read(&mut self) -> io::Result<Self::Value>;

    fn read_exact(&mut self, buf: &mut [Self::Value]) -> io::Result<()>;

    fn chunks(&mut self, chunk_size: usize) -> iter::BinaryChunks<Self> {
        iter::BinaryChunks::new(self, chunk_size)
    }

    fn iter(&mut self) -> iter::BinaryIterator<Self> {
        iter::BinaryIterator::new(self)
    }
}

fn find_prefix<P>(path: P) -> Option<String>
where
    P: Into<PathBuf>,
{
    let string = path
        .into()
        .into_os_string()
        .into_string()
        .expect("cannot convert path to string");

    EXTENSIONS
        .iter()
        .find(|x| string.ends_with(*x))
        .map(|x| string.trim_end_matches(*x).into())
}

pub(crate) fn read_magic<R>(reader: &mut R) -> io::Result<([u8; 8])>
where
    R: io::Read,
{
    let mut magic = [0; 8];

    reader.read_exact(&mut magic)?;

    if magic == MAGIC {
        Ok(magic)
    } else {
        let msg = format!(
            "invalid magic number (expected {:x?}, found {:x?})",
            MAGIC, magic
        );

        Err(io::Error::new(io::ErrorKind::InvalidData, msg))
    }
}

pub(crate) fn parse_magic(magic: &[u8; 8]) -> io::Result<String> {
    match std::str::from_utf8(&magic.to_vec()) {
        Ok(s) => {
            let parsed = s.trim_matches(char::from(0));

            Ok(parsed.to_string())
        },
        Err(_) => {
            let msg = format!("unparseable magic number {:x?}", magic);

            Err(io::Error::new(io::ErrorKind::InvalidData, msg))
        }
    }
}
