use std::collections::HashMap;
use std::fs::read_to_string;

// https://adventofcode.com/2023/day/2

fn main() {
    let mut total_cubes = HashMap::new();
    total_cubes.insert("red", 12);
    total_cubes.insert("green", 13);
    total_cubes.insert("blue", 14);

    let mut games_sum = 0;

    'game: for game in read_to_string("puzzle_input.txt").unwrap().lines() {
        let mut parts = game.split(":");

        let game_number = &(parts.next().unwrap())[5..];
        let game_number: i32 = game_number.parse().unwrap();

        println!("\x1b[1m*** Game #{game_number}\x1b[0m");

        for pull in parts.next().unwrap().split(";") {
            println!("Pull: {pull}");
            for cubes in pull.trim().split(",") {
                if let Some((number, color)) = cubes.trim().split_once(" ") {
                    let number: i32 = number.parse().unwrap();
                    if number > *total_cubes.get(color).unwrap() {
                        println!("\x1b[31mInvalid game, {0} > {1} {color} cubes\x1b[0m", number, total_cubes.get(color).unwrap());
                        continue 'game;
                    }
                } else {
                    panic!("error");
                };
            }
        }
        println!("\x1b[32mValid game\x1b[0m 🙂");
        games_sum += game_number;
    }
    println!("Sum of valid game IDs: {games_sum}");
}
