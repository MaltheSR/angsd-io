use std::{fmt, io};

use crate::saf::constants::MAGIC;

mod entry;
mod reader;

pub use self::{entry::Entry, reader::Reader};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Index {
    n_alleles: usize,
    entries: Vec<Entry>,
}

impl Index {
    pub fn iter_names(&self) -> Names {
        Names::new(self)
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut [Entry] {
        &mut self.entries
    }

    pub fn into_entries(self) -> Vec<Entry> {
        self.entries
    }

    pub fn names(&self) -> Vec<&str> {
        self.entries.iter().map(|x| x.name()).collect()
    }

    pub fn n_alleles(&self) -> usize {
        self.n_alleles
    }

    pub fn n_sites(&self) -> usize {
        self.entries.iter().map(|x| x.n_sites()).sum()
    }

    pub fn new(n_alleles: usize, entries: Vec<Entry>) -> Self {
        Self { n_alleles, entries }
    }

    pub fn from_reader<R>(reader: R) -> io::Result<Self>
    where
        R: io::Read,
    {
        Reader::new(reader).read_index()
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = std::str::from_utf8(&MAGIC)
            .unwrap()
            .trim_matches(char::from(0));
        writeln!(f, "##fileformat={}", version)?;

        writeln!(f, "##alleles={}", self.n_alleles)?;

        for (i, entry) in self.entries.iter().enumerate() {
            write!(f, "{}", entry)?;

            if i + 1 < self.entries.len() {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Names<'a> {
    entries: &'a [Entry],
    current: usize,
    iter: entry::Names<'a>,
}

impl<'a> Names<'a> {
    fn new(index: &'a Index) -> Self {
        let entries = &index.entries;

        Self {
            entries,
            current: 0,
            iter: entries[0].iter_names(),
        }
    }
}

impl<'a> Iterator for Names<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(v),
            None => {
                self.current += 1;

                if self.current >= self.entries.len() {
                    None
                } else {
                    self.iter = self.entries[self.current].iter_names();

                    self.next()
                }
            }
        }
    }
}
