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

    pub fn add_range(&mut self, destination_start: u64, source_start: u64, range_length: u64) {
        self.transformations.push(Conversion::new(destination_start, source_start, range_length));
    }

    pub fn transform(&self, input: &u64) -> Option<(u64, String)> {
        for transformation in &self.transformations {
            let output = transformation.convert(input);
            if output.is_some() {
                return Some((output.unwrap(), self.destination.clone()))
            }
        }
        Some((*input, self.destination.clone()))
    }
}
