#[derive(Debug)]
pub struct Conversion {
    pub destination_start: u32,
    pub source_start: u32,
    pub range_length: u32
}

impl Conversion {
    pub fn new(destination_start: u32, source_start: u32, range_length: u32) -> Conversion {
        Conversion {
            destination_start: destination_start,
            source_start: source_start,
            range_length: range_length
        }
    }

    pub fn convert(&self, input: &u32) -> Option<u32> {
        let range = self.source_start..(self.source_start + self.range_length);
        if range.contains(&input) {
            let position = input - self.source_start;
            return Some(self.destination_start + position);
        }
        None
    }
}