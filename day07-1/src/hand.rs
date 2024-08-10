use crate::card::Card;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Hand {
    pub cards: [Card; 5],
    pub bid: u32,
    pub card_counter: Vec<(Card, u8)>,
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
        let cards_copy = [
            Card::new(input.as_bytes()[0] as char),
            Card::new(input.as_bytes()[1] as char),
            Card::new(input.as_bytes()[2] as char),
            Card::new(input.as_bytes()[3] as char),
            Card::new(input.as_bytes()[4] as char)
        ];
        let mut card_counter: Vec<(Card, u8)> = Vec::new();
        'card_loop: for card in cards_copy {
            for counter in 0..card_counter.len() {
                if card_counter[counter].0 == card {
                    card_counter[counter].1 += 1;
                    continue 'card_loop;
                }
            }
            card_counter.push((card, 1));
        }
        card_counter.sort_by(|a, b| b.1.cmp(&a.1));
        let mut value: u8 = 0;
        if card_counter.len() == 1 {
            // Five of a kind
            value = 7;
        }
        else if card_counter[0].1 == 4 {
            // Four of a kind
            value = 6;
        }
        if card_counter[0].1 == 3 && card_counter[1].1 == 2 {
            // Full house
            value = 5;
        }
        if card_counter[0].1 == 3 && card_counter.len() == 3 {
            // Three of a kind
            value = 4 ;
        }
        if card_counter[0].1 == 2 && card_counter[1].1 == 2 {
            // Two pair
            value = 3;
        }
        if card_counter[0].1 == 2 && card_counter.len() == 4 {
            // One pair
            value = 2;
        }
        if card_counter.len() == 5 {
            // High card
            value = 1;
        }
        Hand {
            cards: cards,
            bid: *bid,
            card_counter: card_counter,
            value: value
        }
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
