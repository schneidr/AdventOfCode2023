use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^[\D]*(\d)(|.*(\d))\D*$").unwrap();
    let mut sum: u32 = 0;

    for line in read_to_string("calibration_data.txt").unwrap().lines() {
        println!("{line}");
        let Some(caps) = re.captures(line) else {
            panic!("no match! in line {line}");
        };
        let first: &str = &caps.get(1).map_or("0", |m| m.as_str());
        let second: &str = &caps.get(3).map_or(first, |m| m.as_str());
        let mut compound: String = String::new();
        compound.push_str(first);
        compound.push_str(second);
        println!("{first} + {second} = {compound}");
        let compound: u32 = compound.parse().unwrap();
        sum += compound;
    }

    println!("***********************");
    println!("Calibration sum: {sum}");
    println!("***********************");
}
