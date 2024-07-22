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
        while line_number < self.schematic.len() {
            println!("Line {line_number}");
            let line: String = self.schematic[line_number].to_string();
            for (index, character) in line
                .chars()
                .enumerate() {
                if character == '*' {
                    let gear_position = index as i32;
                    // 132 123
                    // 123*132
                    // 123 123
                    println!("Gear at position {gear_position}");
                    for y in -1..1 {
                        println!("");
                        for x in -1..1 {
                            let character = self.schematic[(line_number as i32 + y) as usize]
                                .chars()
                                .nth((gear_position+x) as usize)
                                .unwrap();
                            // print!("{character}");
                            if character.is_numeric() {
                                let part_number = String::new();
                                let part_number = self.complete_left(part_number, line_number as i32 + y, gear_position+x);
                                let part_number = self.complete_right(part_number, line_number as i32 + y, gear_position+x);
                                println!("Part number: {part_number}");
                            }
                        }
                    }
                }
            }
            line_number += 1;
        }
    }

    fn complete_left(&mut self, mut part_number: String, line_number: i32, position: i32) -> String {
        let mut substract = 0;
        loop {
            let character = self.schematic[line_number as usize].chars().nth((position - substract) as usize).unwrap();
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
            let character = self.schematic[line_number as usize].chars().nth((position - add) as usize).unwrap();
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
