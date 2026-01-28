use std::fs;

pub fn run(example: bool) {
    let path = "../input/02".to_string();
    let contents = fs::read_to_string(if example { path + "_example" } else { path }).unwrap();
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

enum Choice {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

enum Result {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

struct Round(Choice, Result);

use Choice::{Paper, Rock, Scissor};
use Result::{Draw, Loss, Win};

fn part_1(contents: &str) -> i32 {
    contents
        .split("\n")
        .map(|x| -> Round {
            let mut choices = x.split_ascii_whitespace();
            let opponent = match choices.next().unwrap() {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissor,
                _ => panic!("Invalid input"),
            };
            let second = choices.next().unwrap();
            let mine = match second {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissor,
                _ => panic!("Invalid input"),
            };
            let result = match (opponent, &mine) {
                (Rock, Paper) | (Paper, Scissor) | (Scissor, Rock) => Win,
                (Rock, Scissor) | (Paper, Rock) | (Scissor, Paper) => Loss,
                (Rock, Rock) | (Paper, Paper) | (Scissor, Scissor) => Draw,
            };
            Round(mine, result)
        })
        .map(|round| -> i32 { round.0 as i32 + round.1 as i32 })
        .sum()
}

fn part_2(contents: &str) -> i32 {
    contents
        .split("\n")
        .map(|x| -> Round {
            let mut choices = x.split_ascii_whitespace();
            let opponent = match choices.next().unwrap() {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissor,
                _ => panic!("Invalid input"),
            };
            let second = choices.next().unwrap();
            let result = match second {
                "X" => Loss,
                "Y" => Draw,
                "Z" => Win,
                _ => panic!("Invalid input"),
            };
            let mine = match (opponent, &result) {
                (Scissor, Win) | (Rock, Draw) | (Paper, Loss) => Rock,
                (Rock, Win) | (Paper, Draw) | (Scissor, Loss) => Paper,
                (Paper, Win) | (Scissor, Draw) | (Rock, Loss) => Scissor,
            };
            Round(mine, result)
        })
        .map(|round| -> i32 { round.0 as i32 + round.1 as i32 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use std::fs;

    #[test]
    fn example() {
        let contents = fs::read_to_string("../input/02_example").unwrap();
        assert_eq!(part_1(&contents), 15);
        assert_eq!(part_2(&contents), 12);
    }
}
