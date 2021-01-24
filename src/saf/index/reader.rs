use std::{convert::TryFrom, io};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::saf::read;

use super::{Entry, Index};

pub struct Reader<R> {
    inner: R,
}

impl<R> Reader<R>
where
    R: io::Read,
{
    pub fn new(inner: R) -> Self {
        Self { inner }
    }

    fn read_entry(&mut self) -> io::Result<Entry> {
        // See ANGSD docs: github.com/ANGSD/angsd/blob/master/doc/formats.pdf
        let clen = self.inner.read_u64::<LittleEndian>()?;
        let clen = usize::try_from(clen).expect("cannot convert u64 to usize");

        let mut chr = vec![0u8; clen];
        self.inner.read_exact(&mut chr)?;
        let chr = std::string::String::from_utf8(chr)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid index entry name"))?;

        let nsites = self.inner.read_u64::<LittleEndian>()?;
        let nsites = usize::try_from(nsites).expect("cannot convert u64 to usize");

        let off1 = self.inner.read_u64::<LittleEndian>()?;

        let off2 = self.inner.read_u64::<LittleEndian>()?;

        Ok(Entry::new(chr, nsites, off1, off2))
    }

    pub fn read_index(mut self) -> io::Result<Index> {
        let magic = read::read_magic(&mut self.inner)?;

        let n_categories = self.inner.read_u64::<LittleEndian>()?;
        let n_categories = usize::try_from(n_categories).expect("cannot convert u64 to usize");
        let n_alleles = n_categories + 1;

        let mut entries = Vec::new();
        while let Ok(entry) = self.read_entry() {
            entries.push(entry)
        }

        Ok(Index::new(magic, n_alleles, entries))
    }
}
