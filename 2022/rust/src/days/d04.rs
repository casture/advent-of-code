use std::fs;

pub fn run(example: bool) {
    let path = "../input/04".to_string();
    let contents = fs::read_to_string(if example { path + "_example" } else { path }).unwrap();
    let pairs = fun_name(&contents);
    println!("Part 1: {}", part_1(&pairs));
    println!("Part 2: {}", part_2(&pairs));
}

fn fun_name(contents: &str) -> Vec<Pairs> {
    let pairs: Vec<_> = contents
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|(l, r)| {
                    Pairs(
                        l.split_once("-")
                            .map(|(lower, upper)| {
                                Pair(lower.parse::<i32>().unwrap(), upper.parse::<i32>().unwrap())
                            })
                            .unwrap(),
                        r.split_once("-")
                            .map(|(lower, upper)| {
                                Pair(lower.parse::<i32>().unwrap(), upper.parse::<i32>().unwrap())
                            })
                            .unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();
    pairs
}

struct Pair(i32, i32);
struct Pairs(Pair, Pair);

fn part_1(pairs: &Vec<Pairs>) -> i32 {
    pairs
        .iter()
        .filter(|Pairs(Pair(l_lower, l_upper), Pair(r_lower, r_upper))| {
            if l_lower >= r_lower && l_upper <= r_upper || l_lower <= r_lower && l_upper >= r_upper
            {
                true
            } else {
                false
            }
        })
        .count() as i32
}

fn part_2(pairs: &Vec<Pairs>) -> i32 {
    pairs
        .iter()
        .filter(|Pairs(Pair(l_lower, l_upper), Pair(r_lower, r_upper))| {
            if l_lower >= r_lower && l_lower <= r_upper
                || l_upper <= r_upper && l_upper >= r_lower
                || r_lower >= l_lower && r_lower <= l_upper
                || r_upper <= l_upper && r_upper >= l_lower
            {
                true
            } else {
                false
            }
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::{fun_name, part_1, part_2};
    use std::fs;

    #[test]
    fn example() {
        let contents = fs::read_to_string("../input/04_example").unwrap();
        let pairs = fun_name(&contents);
        assert_eq!(part_1(&pairs), 2);
        assert_eq!(part_2(&pairs), 4);
    }
}
