use crate::map::Map;
use std::fs::read_to_string;
use crate::seed::Seed;

#[derive(Debug)]
pub struct Almanac {
    category_maps: Vec<Map>,
    seeds: Vec<Seed>
}

impl Almanac {
    pub fn new() -> Almanac {
        Almanac {
            category_maps: Vec::new(),
            seeds: Vec::new()
        }
    }

    pub fn init(&mut self, filename: String) {
        let mut current_map: Option<Map> = None;
        for line in read_to_string(filename).unwrap().lines() {
            if line.trim().is_empty() {
                continue;
            }
            if line[..6].eq("seeds:") {
                self.seeds = line[7..]
                    .split_whitespace()
                    .map(|id| Seed::new(id.parse().unwrap()))
                    .collect();
            }
            else if line.trim().ends_with("map:") {
                if current_map.is_some() {
                    self.category_maps.push(current_map.unwrap());
                }
                let map_name: Vec<&str> = line[..line.len()-5].split("-").collect();
                let map = Map::new(map_name[0], map_name[2]);
                current_map = Some(map);
            }
            else {
                let numbers: Vec<u64> = line
                    .trim()
                    .split_whitespace()
                    .map(|num: &str| num.parse().unwrap())
                    .collect();
                let destination_start = numbers[0];
                let source_start = numbers[1];
                let range_length = numbers[2];
                current_map
                    .as_mut()
                    .unwrap()
                    .add_range(destination_start, source_start, range_length);
            }
        }
        if current_map.is_some() {
            self.category_maps.push(current_map.unwrap());
        }
    } 

    fn find_in_map(&self, input: &u64, source: &String) -> Option<u64> {
        for map in &self.category_maps {
            if map.source.eq(source) {
                let (output, destination) = map.transform(input).unwrap();
                if destination == "location" {
                    return Some(output)
                }
                return self.find_in_map(&output, &destination)
            }
        }
        None
    }

    pub fn find_lowest(&self) {
        let source = String::from("seed");
        let mut nearest_seed: Option<&Seed> = None;
        let mut nearest_location: u64 = std::u64::MAX;
        for seed in &self.seeds {
            let result = self.find_in_map(&seed.seed_number, &source);
            if result.is_some() {
                if result.unwrap() < nearest_location {
                    nearest_location = result.unwrap();
                    nearest_seed = Some(seed);
                }
            }
        }
        println!("*************************************");
        println!("Nearest seed: {0} at {1}", nearest_seed.unwrap().seed_number, nearest_location);
        println!("*************************************");
    }

}
