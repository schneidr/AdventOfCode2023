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
        let first: &u32 = &caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        let second: &u32 = &caps.get(3).map_or(*first, |m| m.as_str().parse().unwrap());
        let compound = first*10 + second;

        println!("{first} + {second} = {compound}");
        sum += compound;
    }

    println!("***********************");
    println!("Calibration sum: {sum}");
    println!("***********************");
}
