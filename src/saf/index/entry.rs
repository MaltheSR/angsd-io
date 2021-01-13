use std::fmt;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Entry {
    name: String,
    n_sites: usize,
    position_offset: u64,
    value_offset: u64,
}

impl Entry {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn iter_names(&self) -> Names {
        Names::new(&self.name, self.n_sites)
    }

    pub fn new(name: String, n_sites: usize, position_offset: u64, value_offset: u64) -> Self {
        Self {
            name,
            n_sites,
            position_offset,
            value_offset,
        }
    }

    pub fn n_sites(&self) -> usize {
        self.n_sites
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "##contig=<id={},length={}>", self.name, self.n_sites)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Names<'a> {
    name: &'a str,
    n_sites: usize,
    current: usize,
}

impl<'a> Names<'a> {
    pub fn new(name: &'a str, n_sites: usize) -> Self {
        Self {
            name,
            n_sites,
            current: 0,
        }
    }
}

impl<'a> Iterator for Names<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.n_sites {
            self.current += 1;

            Some(self.name)
        } else {
            None
        }
    }
}
