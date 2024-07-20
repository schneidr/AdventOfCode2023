use std::collections::HashMap;
use std::fs::read_to_string;

// https://adventofcode.com/2023/day/2#part2

fn main() {
    let mut total_cubes = HashMap::new();
    total_cubes.insert("red", 12);
    total_cubes.insert("green", 13);
    total_cubes.insert("blue", 14);

    let mut power_sum = 0;

    'game: for game in read_to_string("puzzle_input.txt").unwrap().lines() {
        let mut parts = game.split(":");

        let game_number = &(parts.next().unwrap())[5..];
        let game_number: i32 = game_number.parse().unwrap();

        println!("\x1b[1m*** Game #{game_number}\x1b[0m");

        let mut num_cubes = HashMap::new();
        num_cubes.insert("red", 0);
        num_cubes.insert("green", 0);
        num_cubes.insert("blue", 0);

        for pull in parts.next().unwrap().split(";") {
            println!("Pull: {pull}");
            for cubes in pull.trim().split(",") {
                if let Some((number, color)) = cubes.trim().split_once(" ") {
                    let number: i32 = number.parse().unwrap();
                    if number > *num_cubes.get(color).unwrap() {
                        num_cubes.entry(color).and_modify(|value| *value = number);
                    }
                } else {
                    panic!("error");
                };
            }
        }
        println!("Highest: {num_cubes:?}");
        let mut power = 1;
        for (key, value) in num_cubes.into_iter() {
            power *= value;
        }
        println!("Power: {power}");
        power_sum += power;
    }
    println!("*** Sum of powers: {power_sum}");
}
