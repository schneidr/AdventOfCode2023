#[derive(Debug)]
pub struct Seed {
    pub seed_number: u64,
}

impl Seed {
    pub fn new(number: u64) -> Seed {
        Seed {
            seed_number: number
        }
    }
}
