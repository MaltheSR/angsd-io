use std::{cmp, io};

use crate::saf;

use super::{Position, SequenceDict};

pub struct MergedSites<I> {
    iters: MultiForward<I>,
    dict: SequenceDict,
}

impl<I> MergedSites<I> {
    fn new(iters: MultiForward<I>, dict: SequenceDict) -> Self {
        Self { iters, dict }
    }
}

impl<'a, R> MergedSites<saf::reader::iter::Sites<'a, R>>
where
    R: io::BufRead,
{
    pub(crate) fn from_safs(safs: &'a mut [saf::reader::Reader<R>]) -> Self {
        let dict = SequenceDict::from_saf_readers(safs);

        let iters = MultiForward(safs.iter_mut().map(|x| Forward(x.sites())).collect());

        Self::new(iters, dict)
    }
}

impl<I, T> Iterator for MergedSites<I>
where
    I: Iterator<Item = io::Result<T>>,
    T: Position,
{
    type Item = io::Result<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next position in each iterator that is contained in the sequence dict
        let mut positions = match self.iters.forward_to_dict(&self.dict)? {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };

        while !positions.all_equal() {
            // Find the greatest position and find the next position in each iterator that is at
            // at least as great
            let argmax = positions.argmax(&self.dict)?;

            // The awkward indexing here is required to appease the borrow checker
            for i in 0..positions.0.len() {
                if !positions.0[i].same_location(&positions.0[argmax]) {
                    positions.0[i] = match self.iters.0[i]
                        .forward_to_position(&positions.0[argmax], &self.dict)?
                    {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e)),
                    };
                }
            }
        }

        Some(Ok(positions.0))
    }
}

struct MultiForward<I>(Vec<Forward<I>>);

impl<I, T> MultiForward<I>
where
    I: Iterator<Item = io::Result<T>>,
    T: Position,
{
    pub fn forward_to_dict(&mut self, dict: &SequenceDict) -> Option<io::Result<Positions<T>>> {
        self.0
            .iter_mut()
            .map(|x| x.forward_to_dict(dict))
            .collect::<Option<io::Result<Vec<T>>>>()
            .map(|x| x.map(Positions))
    }
}

struct Forward<I>(I);

impl<I, T> Forward<I>
where
    I: Iterator<Item = io::Result<T>>,
    T: Position,
{
    pub fn forward_to_dict(&mut self, dict: &SequenceDict) -> Option<io::Result<T>> {
        while let Some(v) = self.0.next() {
            match v {
                Ok(v) => {
                    if dict.contains(v.name()) {
                        return Some(Ok(v));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }

        None
    }

    pub fn forward_to_position(
        &mut self,
        position: &T,
        dict: &SequenceDict,
    ) -> Option<io::Result<T>> {
        while let Some(v) = self.0.next() {
            match v {
                Ok(v) => match dict.compare(&v, position) {
                    Some(cmp::Ordering::Equal) | Some(cmp::Ordering::Greater) => {
                        return Some(Ok(v))
                    }
                    Some(cmp::Ordering::Less) => continue,
                    None => return None,
                },
                Err(e) => return Some(Err(e)),
            }
        }

        None
    }
}

struct Positions<T>(Vec<T>);

impl<T> Positions<T>
where
    T: Position,
{
    pub fn all_equal(&self) -> bool {
        let first = &self.0[0];

        self.0
            .iter()
            .skip(1)
            .all(|x| x.name() == first.name() && x.position() == first.position())
    }

    pub fn argmax(&self, dict: &SequenceDict) -> Option<usize> {
        let mut argmax = 0;

        for (i, position) in self.0.iter().skip(1).enumerate() {
            match dict.compare(position, &self.0[argmax]) {
                Some(cmp::Ordering::Greater) => argmax = i,
                Some(cmp::Ordering::Equal) => (),
                Some(cmp::Ordering::Less) => (),
                None => return None,
            }
        }

        Some(argmax)
    }
}
