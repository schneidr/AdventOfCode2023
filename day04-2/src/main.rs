pub mod scratchcard;
use std::{collections::HashMap, fs::read_to_string};
use crate::scratchcard::scratchcard::Scratchcard;

// https://adventofcode.com/2023/day/4#part2

fn main() {
    let mut all_cards: HashMap<i32, Scratchcard> = HashMap::new();
    let mut my_cards: Vec<i32> = Vec::new();
    for line in read_to_string("scratchcards.txt").unwrap().lines() {
        let card: Scratchcard = Scratchcard::new(&line);
        let card_number = card.card_number;
        all_cards.insert(card.card_number, card);
        my_cards.push(card_number);
    }
    let mut index = 0;
    loop {
        let card_number = my_cards[index];
        let winning_cards: i32 = all_cards.get(&card_number).unwrap().winning_numbers().len() as i32;
        println!("{card_number}: {winning_cards:?}");
        for count in 1..winning_cards+1 {
            let new_card: usize = (card_number + count + 1) as usize;
            my_cards.push(my_cards[new_card]);
        }
        println!("Progress: {index}/{0} cards", my_cards.len());
        index += 1;
        // if index >= 10 {
        if index >= my_cards.len() {
            break;
        }
    }
    println!("Total: {0} cards {my_cards:?}", my_cards.len());
}