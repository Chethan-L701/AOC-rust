pub mod part1 {
    use std::cmp;
    use std::fs;
    pub struct Colors {
        pub red: i32,
        pub blue: i32,
        pub green: i32,
    }
    impl Colors {
        /**
           ## returns a new Object of Dices struct. 
           ```rust
            let dices: Dices = Dices::new();
           ````
           ### with default values:
           - red: 0
           - blue: 0
           - green: 0
         */
        #[allow(dead_code)]
        fn new() -> Self {
            Self {
                red: 0,
                blue: 0,
                green: 0,
            }
        }
        fn from(red: i32, blue: i32, green: i32) -> Self {
            Self { red, blue, green }
        }
    }
    impl std::fmt::Display for Colors {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "RED : {}\nBLUE : {}\nGREEN : {}",
                self.red, self.blue, self.green
            )
        }
    }
    fn parse_round(v: String) -> Colors {
        let dices: Vec<String> = v
            .split(",")
            .filter(|x| x.len() > 0)
            .map(|x| x.trim().to_string())
            .collect();
        let mut n_red: i32 = 0;
        let mut n_blue: i32 = 0;
        let mut n_green: i32 = 0;
        for dice in dices {
            let set: Vec<&str> = dice.trim().split(" ").filter(|x| x.len() > 0).collect();
            let (n_dice, col): (i32, &str) = (set[0].trim().parse().unwrap(), set[1]);
            match col {
                "red" => n_red = n_dice,
                "blue" => n_blue = n_dice,
                "green" => n_green = n_dice,
                _ => continue,
            }
        }
        return Colors::from(n_red, n_blue, n_green);
    }

    pub fn get_max_in_game(game: Vec<String>) -> Colors {
        let mut max = Colors::new();
        for round in game {
            let cur: Colors = parse_round(round);
            (max.red, max.blue, max.green) = (
                cmp::max(max.red, cur.red),
                cmp::max(max.blue, cur.blue),
                cmp::max(max.green, cur.green),
            );
        }
        return max;
    }

    pub fn read_data() -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let f: String = fs::read_to_string("res/day2.txt").expect("couldn't read the file");
        let result: Vec<Vec<String>> = f
            .split("\n")
            .filter(|x| x.len() > 0)
            .map(|x| x.split(":").nth(1).unwrap_or("").trim().to_string())
            .map(|x| x.split(";").map(|x| x.trim().to_string()).collect())
            .collect();
        Ok(result)
    }

    pub fn execute() {
        let data: Vec<Vec<String>> = read_data().unwrap();
        let mut game_id = 0;
        let mut sum = 0;
        for game in data {
            game_id += 1;
            let max: Colors = get_max_in_game(game);
            if max.red > 12 || max.green > 13 || max.blue > 14 {
                continue;
            } else {
                sum += game_id;
            }
        }
        println!("The final sum is : {}", sum);
    }
}

pub mod part2 {
    use crate::day2::part1;
    pub fn execute() {
        let data: Vec<Vec<String>> = part1::read_data().unwrap();
        let mut sum = 0;
        for game in data {
            let max: part1::Colors = part1::get_max_in_game(game);
            sum += max.red * max.blue * max.green;
        }
        println!("The final sum is : {}", sum);
    }
}
