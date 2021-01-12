use std::{fmt, io};

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

    pub fn position(&self) -> u32 {
        self.position
    }

    pub fn values(&self) -> &[f32] {
        &self.values
    }
}

impl fmt::Display for Site<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_values = self
            .values
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\t");

        write!(f, "{}\t{}\t{}", self.name, self.position + 1, fmt_values)
    }
}
