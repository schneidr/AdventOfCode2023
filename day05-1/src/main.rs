pub mod map;
pub mod seed;
use std::fs::read_to_string;
use crate::map::Map;
use crate::seed::Seed;

fn main() {
    let mut category_maps: Vec<Map> = Vec::new();
    let mut current_map: Option<Map> = None;
    for line in read_to_string("almanac.txt").unwrap().lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line[..6].eq("seeds:") {
            let seeds: Vec<Seed> = line[7..]
                .split_whitespace()
                .map(|id| Seed::new(id.parse().unwrap()))
                .collect();
        }
        else if line.trim().ends_with("map:") {
            let map_name: Vec<&str> = line[..line.len()-5].split("-").collect();
            let map = Map::new(map_name[0], map_name[2]);
            current_map = Some(map);
            // category_maps.push(&map);
        }
        else {
            let numbers: Vec<u32> = line
                .trim()
                .split_whitespace()
                .map(|num: &str| num.parse().unwrap())
                .collect();
            let destination_start = numbers[0];
            let source_start = numbers[1];
            let range_length = numbers[2];
            current_map.unwrap().add_range(destination_start, source_start, range_length);
        }
    }
}
