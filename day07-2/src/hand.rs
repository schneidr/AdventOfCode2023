use crate::card::{Card, SORT_ORDER};
use std::cmp::Ordering;
use std::{thread, time};

#[derive(Debug)]
pub struct Hand {
    pub cards: [Card; 5],
    pub bid: u32,
    pub card_counter: Vec<(Card, u8)>,
    cards_copy: [Card; 5],
    pub best_cards: [Card; 5],
    pub value: u8
}

impl Hand {
    pub fn new(input: &str, bid: &u32) -> Hand {
        let cards = [
            Card::new(input.as_bytes()[0] as char),
            Card::new(input.as_bytes()[1] as char),
            Card::new(input.as_bytes()[2] as char),
            Card::new(input.as_bytes()[3] as char),
            Card::new(input.as_bytes()[4] as char)
        ];
        let cards_copy = cards.clone();
        let best_cards = cards.clone();
        Hand {
            cards: cards,
            bid: *bid,
            card_counter: Vec::new(),
            cards_copy: cards_copy,
            best_cards: best_cards,
            value: 0
        }
    }

    pub fn update_value(&mut self, card_index: usize, type_index: usize) {

        if card_index >= self.cards.len() {
            // println!("no cards left in hand");
            return
        }

        // println!("{1}/{2}/{0:?}", self.cards_copy, card_index, type_index);
        // let sleep_time = time::Duration::from_millis(500);
        // thread::sleep(sleep_time);
        let value = self.calculate_value();
        if value > self.value {
            self.value = value;
            self.best_cards = self.cards_copy.clone();
        }

        if self.cards.get(card_index).unwrap().value == 'J' {
            // println!("card {card_index} is Joker");
            if type_index >=SORT_ORDER.len() {
                return
            }
            self.cards_copy[card_index] = Card::new(SORT_ORDER[type_index]);
            self.update_value(card_index, type_index+1);
        }

        self.update_value(card_index+1, 1);


    }
    fn calculate_value(&mut self) -> u8 {
        let mut card_counter: Vec<(Card, u8)> = Vec::new();
        'card_loop: for card in &self.cards_copy {
            for counter in 0..card_counter.len() {
                if card_counter[counter].0 == *card {
                    card_counter[counter].1 += 1;
                    continue 'card_loop;
                }
            }
            card_counter.push((card.clone(), 1));
        }
        card_counter.sort_by(|a, b| b.1.cmp(&a.1));
        if card_counter.len() == 1 {
            // Five of a kind
            return 7
        }
        if card_counter[0].1 == 4 {
            // Four of a kind
            return 6
        }
        if card_counter[0].1 == 3 && card_counter[1].1 == 2 {
            // Full house
            return 5
        }
        if card_counter[0].1 == 3 && card_counter.len() == 3 {
            // Three of a kind
            return 4
        }6632
        if card_counter[0].1 == 2 && card_counter[1].1 == 2 {
            // Two pair
            return 3
        }
        if card_counter[0].1 == 2 && card_counter.len() == 4 {
            // One pair
            return 2
        }
        if card_counter.len() == 5 {
            // High card
            return 1
        }
        0
    }

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value == other.value {
            for index in 0..self.cards.len() {
                let order = self.cards.get(index).unwrap().cmp(other.cards.get(index).unwrap());
                if order.is_eq() {
                    continue
                }
                return order
            }
        }
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Hand { }
