use std::collections::HashMap;
use std::fs::read_to_string;

// https://adventofcode.com/2023/day/2

let mut total_cubes = HashMap::new();
total_cubes.insert("red", 12);
total_cubes.insert("green", 13);
total_cubes.insert("blue", 14);

fn main() {
    for line in read_to_string("puzzle_input.txt").unwrap().lines() {
        let mut parts = line.split(":");

        let game_number = &(parts.next().unwrap())[5..];
        let game_number: i32 = game_number.parse().unwrap();

        println!("*** Game #{game_number}");

        let mut num_cubes = HashMap::new();
        num_cubes.insert("red", 0);
        num_cubes.insert("green", 0);
        num_cubes.insert("blue", 0);

        for pull in parts.next().unwrap().split(";") {
            println!("{pull:?}");
            
        }

    }
}
