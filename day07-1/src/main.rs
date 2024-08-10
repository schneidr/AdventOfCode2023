pub mod card;
pub mod hand;
use crate::hand::Hand;
use std::fs::read_to_string;

fn main() {
    // let hands: Vec<Hand> = Vec::new();
    for line in read_to_string("bids.txt").unwrap().lines() {
        let mut parts = line.split_whitespace();
        let cards = parts.next().unwrap();
        let bid: u32 = parts.next().unwrap().parse().unwrap();
        let hand = Hand::new(cards);
        let value = hand.get_value();
        println!("Value: {value}");
        // TODO implement ranking
        // TODO calculate bids
        // TODO calculate winnings
    }    
}
