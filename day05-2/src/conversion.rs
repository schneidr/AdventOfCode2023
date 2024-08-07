#[derive(Debug)]
pub struct Conversion {
    pub destination_start: u64,
    pub source_start: u64,
    pub range_length: u64
}

impl Conversion {
    pub fn new(destination_start: u64, source_start: u64, range_length: u64) -> Conversion {
        Conversion {
            destination_start: destination_start,
            source_start: source_start,
            range_length: range_length
        }
    }

    pub fn convert(&self, input: &u64) -> Option<u64> {
        let range = self.source_start..(self.source_start + self.range_length);
        if range.contains(&input) {
            let position = input - self.source_start;
            return Some(self.destination_start + position);
        }
        None
    }
}