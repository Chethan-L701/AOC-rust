pub mod part1 {
    use std::fs;

    pub struct Card {
        pub winning: Vec<i32>,
        pub my_num: Vec<i32>,
    }
    impl Card {
        #[allow(dead_code)]
        fn new() -> Self {
            Self {
                winning: Vec::new(),
                my_num: Vec::new(),
            }
        }
        fn from(winning: Vec<i32>, my_num: Vec<i32>) -> Self {
            Self { winning, my_num }
        }
    }
    impl std::fmt::Display for Card {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Card :\nWinning Numbers : {:?}\nMy Numbers : {:?}\n",
                self.winning, self.my_num
            )
        }
    }

    pub fn parse_file() -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let f: String = fs::read_to_string("res/day4.txt").unwrap();
        let result: Vec<Vec<String>> = f
            .trim()
            .split("\n")
            .filter(|x| x.len() > 0)
            .map(|x| {
                x.trim()
                    .split(":")
                    .nth(1)
                    .unwrap_or("")
                    .trim()
                    .split("|")
                    .map(|x| x.trim().to_string())
                    .collect()
            })
            .collect();

        Ok(result)
    }

    pub fn to_cards(data: Vec<Vec<String>>) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        for card in data {
            let winning: Vec<i32> = card[0]
                .trim()
                .split(" ")
                .filter(|x| x.len() > 0)
                .map(|x| x.trim().parse().unwrap())
                .collect();
            let my_num: Vec<i32> = card[1]
                .trim()
                .split(" ")
                .filter(|x| x.len() > 0)
                .map(|x| x.trim().parse().unwrap())
                .collect();
            cards.push(Card::from(winning, my_num));
        }
        return cards;
    }

    fn score_per_game(card: Card) -> i32 {
        let (winning, nums) = (card.winning, card.my_num);
        let mut score: i32 = 0;
        for win_num in winning {
            if nums.contains(&win_num) {
                score = if score == 0 { 1 } else { score * 2 };
            }
        }
        return score;
    }

    pub fn execute() {
        let cards = to_cards(parse_file().unwrap());
        let mut total_score: i32 = 0;
        for card in cards {
            total_score += score_per_game(card);
        }
        println!("The Final score is : {}", total_score);
    }
}

pub mod part2 {
    use crate::day4::part1;
    use std::collections::HashMap;

    fn wins_per_game(card: part1::Card) -> i32 {
        let (winning, nums) = (card.winning, card.my_num);
        let mut wins: i32 = 0;
        for win_num in winning {
            if nums.contains(&win_num) {
                wins += 1;
            }
        }
        return wins;
    }
    pub fn execute() {
        let cards = part1::to_cards(part1::parse_file().unwrap());
        let mut cards_hash: HashMap<i32, i32> = HashMap::new();
        cards_hash.insert(1, 1);
        let mut card_id = 1;
        for card in cards {
            let wins = wins_per_game(card);
            let n_cards = match cards_hash.get(&card_id) {
                Some(value) => *value ,
                None => 0,
            };
            for i in (card_id + 1)..=(card_id + wins) {
                if let Some(&value) = cards_hash.get(&i) {
                    cards_hash.insert(i, value + n_cards);
                } else {
                    cards_hash.insert(i, 2);
                }
            }
            card_id += 1;
        }

        let final_sum: i32 = cards_hash.values().sum();
        println!("The total numbers of cards is : {}", final_sum);
    }
}
