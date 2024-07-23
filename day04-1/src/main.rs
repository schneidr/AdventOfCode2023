use std::fs::read_to_string;

// https://adventofcode.com/2023/day/4

fn main() {
    let mut total_worth = 0;
    for card in read_to_string("scratchcards.txt").unwrap().lines() {
        let mut worth = 0;
        let mut parts = card.split(":");
        let card_number = parts.next().unwrap();
        print!("{card_number}");
        let mut number_lists = parts.next().unwrap().split("|");
        let winning_numbers: Vec<&str> = number_lists.next().unwrap().trim().split_whitespace().collect();
        // println!("{winning_numbers:?}");
        let my_numbers: Vec<&str> = number_lists.next().unwrap().trim().split_whitespace().collect();
        // println!("{my_numbers:?}");
        for win_number in winning_numbers {
            if my_numbers.contains(&win_number) {
                if worth == 0 {
                    worth = 1;
                }
                else {
                    worth *= 2;
                }
            }
        }
        println!(": {worth} points");
        total_worth += worth;
    }
    println!("*************************");
    println!("Total worth: {total_worth}");
    println!("*************************");
}
