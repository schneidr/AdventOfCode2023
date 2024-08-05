use std::process::Output;

use crate::conversion::Conversion;

#[derive(Debug)]
pub struct Map {
    pub source: String,
    pub destination: String,
    transformations: Vec<Conversion>
}

impl Map {
    pub fn new(source: &str, destination: &str) -> Map {
        Map {
            source: String::from(source),
            destination: String::from(destination),
            transformations: Vec::new()
        }
    }

    pub fn add_range(&mut self, destination_start: u32, source_start: u32, range_length: u32) {
        self.transformations.push(Conversion::new(destination_start, source_start, range_length));
    }

    pub fn transform(self, input: u32) -> Option<(u32, String)> {
        for transformation in self.transformations {
            let output = transformation.convert(input);
            if output.is_some() {
                return Some((output.unwrap(), self.destination))
            }
        }
        None
    }
}
