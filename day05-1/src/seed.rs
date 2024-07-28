#[derive(Debug)]
pub struct Seed {
    pub seed_number: u32,
}

impl Seed {
    pub fn new(number: u32) -> Seed {
        Seed {
            seed_number: number
        }
    }
}
