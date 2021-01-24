use std::cmp;

use indexmap::set::IndexSet;

use crate::saf;

use super::Position;

pub(crate) struct SequenceDict(IndexSet<String>);

impl SequenceDict {
    pub fn compare<T>(&self, first: &T, second: &T) -> Option<cmp::Ordering>
    where
        T: Position,
    {
        if first.name() == second.name() {
            Some(first.position().cmp(&second.position()))
        } else {
            let first_genome_position = self.0.get_index_of(first.name())?;
            let second_genome_position = self.0.get_index_of(second.name())?;

            Some(first_genome_position.cmp(&second_genome_position))
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.contains(name)
    }

    pub fn from_saf_readers<R>(readers: &[saf::read::Reader<R>]) -> Self {
        let indices = readers.iter().map(|x| x.index()).collect::<Vec<_>>();

        let mut set = IndexSet::<String>::default();

        for (i, index) in indices.iter().enumerate() {
            let new_set: IndexSet<String> = index
                .entries()
                .iter()
                .map(|x| x.name().to_string())
                .collect();

            if i == 0 {
                set = new_set;
            } else {
                set.retain(|x| new_set.contains(x));
            }
        }

        Self(set)
    }
}
