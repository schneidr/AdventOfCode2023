use std::cmp::Ordering;

#[derive(Debug)]
pub struct Card {
    pub value: char,
    sort_order: [char; 13]
}

impl Card {
    pub fn new(value: &char) -> Card {
        Card {
            value: *value,
            sort_order: ['2','3','4','5','6','7','8','9','T','J','Q','K','A']
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let index = self.sort_order.iter().position(|&r| r == self.value);
        let other_index = self.sort_order.iter().position(|&r| r == other.value);
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
