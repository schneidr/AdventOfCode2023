use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    pub cards: [Card; 5],
}

impl Hand {
    pub fn new(input: &str) -> Hand {
        Hand {
            cards: [
                Card::new(input.as_bytes()[0] as char),
                Card::new(input.as_bytes()[1] as char),
                Card::new(input.as_bytes()[2] as char),
                Card::new(input.as_bytes()[3] as char),
                Card::new(input.as_bytes()[4] as char)
            ]
        }
    }

    pub fn get_value(&self) -> u8 {
        let mut card_counter: Vec<(&Card, u8)> = Vec::new();
        'card_loop: for card in &self.cards {
            for counter in 0..card_counter.len() {
                if card_counter[counter].0 == card {
                    card_counter[counter].1 += 1;
                    continue 'card_loop;
                }
            }
            card_counter.push((card, 1));
        }
        card_counter.sort_by(|a, b| b.1.cmp(&a.1));
        println!("{card_counter:?}");
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
        }
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

    // TODO implement order
}
