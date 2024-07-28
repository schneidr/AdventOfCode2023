use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    pub source: String,
    pub destination: String,
    transformations: HashMap<u32, u32>
}

impl Map {
    pub fn new(source: &str, destination: &str) -> Map {
        Map {
            source: String::from(source),
            destination: String::from(destination),
            transformations: HashMap::new()
        }
    }

    pub fn add_range(&mut self, destination_start: u32, source_start: u32, range_length: u32) {
        for counter in 0..range_length {
            self.transformations.insert(source_start + counter, destination_start + counter);
        }
    }
}
