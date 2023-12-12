pub mod part1 {
    use std::fs;
    pub struct Race {
        pub time: Vec<i64>,
        pub dist: Vec<i64>,
    }
    impl Race {
        pub fn from(time: Vec<i64>, dist: Vec<i64>) -> Self {
            Self { time, dist }
        }
    }
    pub fn parse_file() -> Race {
        let f: String = fs::read_to_string("res/day6.txt").unwrap();
        let result: Vec<Vec<i64>> = f
            .trim()
            .split("\n")
            .filter(|x| x.len() > 0)
            .map(|x| {
                x.trim()
                    .split(":")
                    .nth(1)
                    .unwrap_or("")
                    .trim()
                    .split(" ")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.trim().parse().unwrap())
                    .collect()
            })
            .collect();
        let (time, dist) = (result[0].clone(), result[1].clone());
        return Race::from(time, dist);
    }
    pub fn possible_completions(race: Race) -> i64 {
        let mut pos: Vec<i64> = Vec::new();
        let length = race.time.len();
        for i in 0..length {
            let mut cur = 0;
            for j in 0..=race.time[i] {
                if j * (race.time[i] - j) > race.dist[i] {
                    cur += 1;
                }
            }
            pos.push(cur);
        }
        return pos.iter().fold(1, |acc, x| acc * x);
    }
    pub fn execute() {
        println!(
            "The Final answer is : {}",
            possible_completions(parse_file())
        );
    }
}

pub mod part2 {
    use super::part1;
    use std::fs;
    pub fn parse_file() -> part1::Race {
        let f: String = fs::read_to_string("res/day6.txt").unwrap();
        let result: Vec<i64> = f
            .trim()
            .split("\n")
            .filter(|x| x.len() > 0)
            .map(|x| {
                x.trim()
                    .split(":")
                    .nth(1)
                    .unwrap_or_default()
                    .trim()
                    .split(" ")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.trim())
                    .fold(String::new(), |acc, s| acc + s)
                    .trim()
                    .parse()
                    .unwrap()
            })
            .collect();
        let (time, dist) = (vec![result[0]], vec![result[1]]);
        return part1::Race::from(time, dist);
    }
    pub fn execute() {
        println!(
            "The Final answer is : {}",
            part1::possible_completions(parse_file())
        );
    }
}
