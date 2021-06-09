use std::{fmt, io};

use crate::merge::Position;

#[derive(Clone, Debug, Default)]
pub struct Site<'a> {
    name: &'a str,
    position: u32,
    values: Vec<f32>,
}

impl<'a> Site<'a> {
    pub(crate) fn from_io(
        name: &'a str,
        position: io::Result<u32>,
        values: io::Result<Vec<f32>>,
    ) -> io::Result<Self> {
        Ok(Self {
            name,
            position: position?,
            values: values?,
        })
    }

    pub fn into_values(self) -> Vec<f32> {
        self.values
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn new(name: &'a str, position: u32, values: Vec<f32>) -> Self {
        Self {
            name,
            position,
            values,
        }
    }

    pub fn values(&self) -> &[f32] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [f32] {
        &mut self.values
    }

    pub fn position(&self) -> u32 {
        self.position
    }

    pub fn same_location(&self, other: &Self) -> bool {
        self.name() == other.name() && self.position() == other.position()
    }
}

impl Position for Site<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn position(&self) -> u32 {
        self.position
    }

    fn same_location(&self, other: &Self) -> bool {
        self.same_location(other)
    }
}

impl fmt::Display for Site<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_values = self
            .values
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        write!(f, "{}\t{}\t{}", self.name, self.position + 1, fmt_values)
    }
}
