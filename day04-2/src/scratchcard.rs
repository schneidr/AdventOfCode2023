pub mod scratchcard {

    #[derive(Debug)]
    pub struct Scratchcard {
        card_number: i32,
        winning_numbers: Vec<i32>,
        card_numbers: Vec<i32>
    }

    impl Scratchcard {
        pub fn new(input: &str) -> Scratchcard {
            println!("{input:?}");
            let card_number: i32 = input[5..8].trim().parse().unwrap();
            let winning_numbers: Vec<i32> = input[10..40]
                .trim()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let card_numbers: Vec<i32> = input[42..]
                .trim()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            println!("{winning_numbers:?}");
            Scratchcard {
                card_number: card_number,
                winning_numbers: winning_numbers,
                card_numbers: card_numbers
            }
        }
    } 
}