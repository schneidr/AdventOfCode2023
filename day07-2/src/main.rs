pub mod card;
pub mod hand;
use crate::hand::Hand;
use std::fs::read_to_string;

fn main() {
    let mut hands: Vec<Hand> = Vec::new();
    for line in read_to_string("bids.txt").unwrap().lines() {
        let mut parts = line.split_whitespace();
        let cards = parts.next().unwrap();
        let bid: u32 = parts.next().unwrap().parse().unwrap();
        let mut hand = Hand::new(cards, &bid);
        hand.update_value(0, 1);
        println!("best value: {0} {1:?}", hand.value, hand.best_cards);
        hands.push(hand);
    }    
    hands.sort();
    let mut total_winnings: u64 = 0;
    for (position, hand) in hands.iter().enumerate() {
        let winnings = ((position as u64)+1) * hand.bid as u64;
        total_winnings += winnings;
    }
    println!("Total winnings: {total_winnings}");
}
