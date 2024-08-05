pub mod almanac;
pub mod conversion;
pub mod map;
pub mod seed;
use crate::almanac::Almanac;


fn main() {
    let mut almanac = Almanac::new();
    almanac.init(String::from("almanac.txt"));
}
