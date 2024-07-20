use std::{
    fs::File,
    io::{prelude::*, BufReader}
};
use regex::Regex;

fn main() {
    const LINE_LENGTH: i32 = 140;

    let mut used_parts: Vec<i32> = Vec::new();

    let file = File::open("schematic.txt").expect("no such file");
    let buf = BufReader::new(file);
    let schematic: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).collect();

    let regex_number = Regex::new(r"(\d+)").unwrap();
    let regex_symbol = Regex::new(r"[^\d\.]").unwrap();
    for (line_number, line) in schematic.iter().enumerate() {
        for capture in regex_number.captures_iter(line) {
            let position = capture.get(0).unwrap();
            // println!("{position:?}");
            let part_number: i32 = position.as_str().parse().unwrap();
            let mut start: i32 = position.start() as i32 - 1;
            if start < 0 {
                start = 0
            }
            let start = start as usize;
            let mut end: i32 = position.end() as i32 + 1;
            if end > LINE_LENGTH {
                end = LINE_LENGTH
            }
            let end = end as usize;
            if regex_symbol.is_match(&line[start..end]) {
                used_parts.push(part_number);
            }
            if line_number > 0 {
                if regex_symbol.is_match(&schematic[line_number-1][start..end]) {
                    used_parts.push(part_number);
                }
            }
            if line_number < schematic.len() - 2 {
                if regex_symbol.is_match(&schematic[line_number+1][start..end]) {
                    used_parts.push(part_number);
                }
            }
        };
    }
    let sum_parts: i32 = used_parts.iter().sum();
    println!("*************");
    println!("Sum: {0}", sum_parts);
    println!("*************");
}
