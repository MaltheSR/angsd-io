use std::io;

#[cfg(feature = "ndarray")]
use ndarray::prelude::*;

use crate::merge::MergedSites;

use super::*;

pub struct MergedReader<R> {
    readers: Vec<Reader<R>>,
}

impl<R> MergedReader<R>
where
    R: io::BufRead,
{
    pub fn iter(&mut self) -> MergedSites<iter::Sites<R>> {
        MergedSites::from_safs(&mut self.readers)
    }

    pub fn new(readers: Vec<Reader<R>>) -> Self {
        Self { readers }
    }

    #[cfg(feature = "ndarray")]
    pub fn read_values_to_arrays(mut self) -> io::Result<Vec<Array2<f32>>> {
        let dims: Vec<(usize, usize)> = self
            .readers
            .iter()
            .map(|reader| reader.index())
            .map(|index| (index.n_sites(), index.n_alleles()))
            .collect();

        let min_n_sites = dims.iter().map(|dim| dim.0).min().unwrap();

        // The highest possible number of merged sites is equal to the smallest number of sites in
        // the input files, and we pre-allocate space accordingly. When very few sites intersect,
        // this is wasteful, but this should rarely outweigh the benefits of pre-allocation.
        let mut vecs: Vec<Vec<f32>> = dims
            .iter()
            .map(|dim| Vec::with_capacity(min_n_sites * dim.1))
            .collect();

        for multisite in self.iter() {
            for (i, site) in multisite?.into_iter().enumerate() {
                vecs[i].append(&mut site.into_values())
            }
        }

        Ok(vecs
            .into_iter()
            .zip(dims.iter())
            .map(|(vec, dim)| {
                let n_cols = dim.1;
                let n_rows = vec.len() / n_cols;
                Array::from_shape_vec((n_rows, n_cols), vec).unwrap()
            })
            .collect())
    }
}
