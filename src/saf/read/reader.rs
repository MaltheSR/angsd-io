use std::{fs::File, io, path::Path};

#[cfg(feature = "ndarray")]
use ndarray::{Array, Array2};

use super::{iter, BinaryRead, PositionReader, ValueReader};

use crate::{
    saf::{
        self,
        constants::{INDEX_EXTENSION, POSITION_EXTENSION, VALUE_EXTENSION},
        index,
    },
    utils,
};

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
        let n_sites = self.index.n_sites();

        iter::Sites::new(
            self.index.iter_names(),
            self.position_reader.iter(),
            self.value_reader.chunks(self.index.n_alleles()),
            n_sites,
        )
    }

    pub fn read_header(&mut self) -> io::Result<String> {
        let index_header = utils::parse_magic(&self.index.magic())?;
        let position_header = self.position_reader.read_header()?;
        let value_header = self.value_reader.read_header()?;

        assert!(index_header == position_header && index_header == value_header);

        Ok(index_header)
    }
}

impl Reader<io::BufReader<File>> {
    pub fn from_paths<P>(index_path: &P, position_path: &P, value_path: &P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let index = File::open(index_path).map(|x| index::Reader::new(x).read_index())??;

        let position_reader = PositionReader::from_path(position_path)?;

        let value_reader = ValueReader::from_path(value_path)?;

        Ok(Self::new(index, position_reader, value_reader))
    }

    pub fn from_member_path<P>(path: &P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let prefix = saf::utils::prefix(path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "input is not a SAF member path",
            )
        })?;

        Self::from_prefix(&prefix)
    }

    pub fn from_prefix<P>(prefix: &P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let prefix = prefix.as_ref();

        let index_path = prefix.with_extension(INDEX_EXTENSION);

        let value_path = prefix.with_extension(VALUE_EXTENSION);

        let position_path = prefix.with_extension(POSITION_EXTENSION);

        Self::from_paths(&index_path, &position_path, &value_path)
    }
}
