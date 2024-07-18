use std::collections::HashMap;
use std::fs::read_to_string;
// use regex::Regex;

fn main() {
    // let re = Regex::new(r"^[\D]*(\d)(|.*(\d))\D*$").unwrap();
    // let mut sum: u32 = 0;

    let mut numbers = HashMap::new();
    numbers.insert("one", 1);
    numbers.insert("two", 2);
    numbers.insert("three", 3);
    numbers.insert("four", 4);
    numbers.insert("five", 5);
    numbers.insert("six", 6);
    numbers.insert("seven", 7);
    numbers.insert("eight", 8);
    numbers.insert("nine", 9);
    numbers.insert("1", 1);
    numbers.insert("2", 2);
    numbers.insert("3", 3);
    numbers.insert("4", 4);
    numbers.insert("5", 5);
    numbers.insert("6", 6);
    numbers.insert("7", 7);
    numbers.insert("8", 8);
    numbers.insert("9", 9);

    for line in read_to_string("calibration_data.txt").unwrap().lines() {
        println!("{line}");

        let mut first = 100;
        let mut last = -1;

        for (key,value) in &numbers {
            let mut occurences: Vec<(usize, &str)> = line.match_indices(key).collect();
            occurences.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let first = occurences.first();
            if ! first.is_none() {
                // first = 
                // Continue here
                println!("{first:?}");
            }
            let last = occurences.last();
            println!("{last:?}");
        }

        // let Some(caps) = re.captures(line) else {
        //     panic!("no match! in line {line}");
        // };
        // let first: &u32 = &caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        // let second: &u32 = &caps.get(3).map_or(*first, |m| m.as_str().parse().unwrap());
        // let compound = first*10 + second;

        // println!("{first} + {second} = {compound}");
        // sum += compound;
    }

    // https://adventofcode.com/2023/day/1#part2

    // println!("***********************");
    // println!("Calibration sum: {sum}");
    // println!("***********************");
}
