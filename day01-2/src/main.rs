use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let mut sum: u32 = 0;

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

    let mut counter = 1;
    for line in read_to_string("calibration_data.txt").unwrap().lines() {
        println!("{line}");

        let mut first_occurrence: i32 = 100;
        let mut first_num = 0;
        let mut last_occurrence: i32 = -1;
        let mut last_num = 0;

        for (key,value) in &numbers {
            let mut occurrences: Vec<(usize, &str)> = line.match_indices(key).collect();
            occurrences.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let first = occurrences.first();
            if ! first.is_none() {
                let first = first.ok_or("error").unwrap();
                if (first.0 as i32) < first_occurrence {
                    first_occurrence = (first.0 as i32);
                    first_num = *value;
                }
            }
            let last = occurrences.last();
            if ! last.is_none() {
                let last = last.ok_or("error").unwrap();
                if (last.0 as i32) > last_occurrence {
                    last_occurrence = (last.0 as i32);
                    last_num = *value;
                }
            }
        }

        if first_num == 0 || last_num == 0 {
            panic!("number can't be 0: {first_num}, {last_num}");
        }
        let compound = first_num*10 + last_num;

        println!("{counter}: {first_num} + {last_num} = {compound}");
        sum += compound;
        counter += 1;
    }

    // https://adventofcode.com/2023/day/1#part2

    println!("************************");
    println!(" Calibration sum: {sum}");
    println!("************************");
}
