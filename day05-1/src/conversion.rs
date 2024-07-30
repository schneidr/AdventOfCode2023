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
}