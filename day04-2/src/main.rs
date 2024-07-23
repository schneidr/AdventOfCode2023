pub mod scratchcard;
use std::fs::read_to_string;
use crate::scratchcard::scratchcard::Scratchcard;


// https://adventofcode.com/2023/day/4#part2

fn main() {
    let mut my_cards: Vec<Scratchcard> = Vec::new();
    for line in read_to_string("scratchcards.txt").unwrap().lines() {
        let card: Scratchcard = Scratchcard::new(&line);
        my_cards.push(card);
    }
    println!("{my_cards:?}");
}