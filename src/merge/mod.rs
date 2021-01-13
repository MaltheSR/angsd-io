mod iter;
mod sequence_dict;

pub use iter::MergedSites;
pub(crate) use sequence_dict::SequenceDict;

pub trait Position {
    fn name(&self) -> &str;

    fn position(&self) -> u32;

    fn same_location(&self, other: &Self) -> bool;
}
