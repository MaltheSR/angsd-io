use std::io;

use crate::merge::MergedSites;

use super::*;

pub struct MergedReader<R> {
    readers: Vec<Reader<R>>,
}

impl<R> MergedReader<R>
where
    R: io::BufRead,
{
    pub fn iter<'a>(&'a mut self) -> MergedSites<iter::Sites<'a, R>> {
        MergedSites::from_safs(&mut self.readers)
    }

    pub fn new(readers: Vec<Reader<R>>) -> Self {
        Self { readers }
    }
}