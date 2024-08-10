use std::cmp::Ordering;

const SORT_ORDER: [char; 13] = ['J','2','3','4','5','6','7','8','9','T','Q','K','A'];

#[derive(Debug)]
pub struct Card {
    pub value: char
}

impl Card {
    pub fn new(value: char) -> Card {
        Card {
            value: value
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let index = SORT_ORDER.iter().position(|&r| r == self.value);
        let other_index = SORT_ORDER.iter().position(|&r| r == other.value);
        index.cmp(&other_index)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Card { }
