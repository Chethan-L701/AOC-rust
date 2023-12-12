pub mod part1 {
    use std::fs;
    fn find_number(chars: &Vec<char>) -> String {
        for ch in chars {
            if ch.is_digit(10) {
                return ch.to_string();
            }
        }
        return String::new();
    }
    fn find_last_number(chars: &Vec<char>) -> String {
        let mut rev = chars.clone();
        rev.reverse();
        for ch in rev {
            if ch.is_digit(10) {
                return ch.to_string();
            }
        }
        return String::new();
    }
    pub fn execute() {
        println!("Hello, World!");
        let data = fs::read_to_string("res/day1.txt").expect("couldn't read the file.");
        let lines = data.split("\n").filter(|x| x.len() > 0);
        let mut numbers: i32 = 0;
        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            let num = format!("{}{}", find_number(&chars), find_last_number(&chars));
            let num: i32 = num.trim().parse().unwrap();
            numbers += num;
        }
        println!("The final number is : {numbers}");
    }
}

pub mod part2 {
    use std::fs;
    struct Map {
        val: String,
        s: String,
    }
    impl Map {
        fn from(s: &str, val: &str) -> Self {
            return Map {
                s: String::from(s),
                val: String::from(val),
            };
        }
    }
    fn find_str_num(val: String) -> String {
        let nums: Vec<Map> = vec![
            Map::from("one", "1"),
            Map::from("two", "2"),
            Map::from("three", "3"),
            Map::from("four", "4"),
            Map::from("five", "5"),
            Map::from("six", "6"),
            Map::from("seven", "7"),
            Map::from("eight", "8"),
            Map::from("nine", "9"),
        ];
        for num in nums {
            if val.len() >= num.s.len() && &val[..num.s.len()] == &num.s {
                return num.val;
            }
        }
        return String::new();
    }
    pub fn execute() {
        println!("Hello, World!");
        let data = fs::read_to_string("res/day1.txt").expect("couldn't read the file.");
        let lines = data.split("\n").filter(|x| x.len() > 0);
        let mut sum: i64 = 0;
        for mut line in lines {
            let mut number = String::new();
            while line.len() > 0 {
                if line.chars().next().unwrap_or(' ').is_digit(10) {
                    number += &line[..1];
                } else {
                    number = format!("{number}{}", find_str_num(line.to_string()));
                }
                line = &line[1..];
            }
            let number = if number.len() == 1 {
                format!("{number}{number}")
            } else {
                format!(
                    "{}{}",
                    number.chars().next().unwrap_or(' '),
                    number.chars().last().unwrap_or(' ')
                )
            };
            let num: i64 = number.trim().parse().unwrap();
            sum += num;
        }
        println!("The final sum is : {}", sum);
    }
}
