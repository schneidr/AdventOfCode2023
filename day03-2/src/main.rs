use std::{
    fs::File,
    io::{prelude::*, BufReader}
};


pub struct GearChecker {
    schematic: Vec<String>
}

impl GearChecker {

    pub fn new() -> GearChecker {
        GearChecker {
            schematic: Vec::new()
        }
    }

    pub fn load(&mut self, filename: &str) {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        self.schematic = buf.lines().map(|l| l.expect("Could not read line")).collect();
    }

    pub fn find_gears(&mut self) {
        let mut line_number = 0;
        let mut ratio_sum: i32 = 0;
        while line_number < self.schematic.len() {
            println!("Line {line_number}");
            let line: String = self.schematic[line_number].to_string();
            for (index, character) in line
                .chars()
                .enumerate() {
                if character == '*' {
                    let gear_position = index as i32;
                    // 123 123
                    // 123*132
                    // 123 123
                    println!("Possible gear at position {gear_position}");
                    let mut parts: Vec<i32> = Vec::new();
                    for y in -1..1 {
                        for x in -1..1 {
                            let character = self.schematic[(line_number as i32 + y) as usize]
                                .chars()
                                .nth((gear_position+x) as usize)
                                .unwrap();
                            if character.is_numeric() {
                                let part_number = String::new();
                                let part_number = self.complete_left(part_number, line_number as i32 + y, gear_position+x);
                                let part_number = self.complete_right(part_number, line_number as i32 + y, gear_position+x+1);
                                println!("Part number: {part_number}");
                                let part_number: i32 = part_number.parse().unwrap();
                                if ! parts.contains(&part_number) {
                                    parts.push(part_number);
                                }
                            }
                        }
                    }
                    if parts.len() == 2 {
                        let gear_ratio: i32 = parts[0] * parts[1];
                        println!("Gear found! Ratio is {gear_ratio}");
                        ratio_sum += gear_ratio;
                    }
                }
            }
            line_number += 1;
        }
        println!("************************");
        println!("Sum of all gear ratios: {ratio_sum}");
        println!("************************");
    }

    fn complete_left(&mut self, mut part_number: String, line_number: i32, position: i32) -> String {
        let mut substract = 0;
        loop {
            if position + substract < 0 {
                return part_number
            }
            let character = self.schematic[line_number as usize].chars().nth((position + substract) as usize).unwrap();
            if ! character.is_numeric() {
                return part_number
            }
            part_number.insert(0, character);
            substract -= 1;
        }
    }

    fn complete_right(&mut self, mut part_number: String, line_number: i32, position: i32) -> String {
        let mut add = 0;
        loop {
            if position + add > (self.schematic[line_number as usize].len() as i32 - 1) {
                return part_number
            }
            let character = self.schematic[line_number as usize].chars().nth((position + add) as usize).unwrap();
            if ! character.is_numeric() {
                return part_number
            }
            part_number.insert(0, character);
            add += 1;
        }
    }

}


fn main() {
    let mut checker = GearChecker::new();
    
    checker.load("schematic.txt");
    checker.find_gears();

}
