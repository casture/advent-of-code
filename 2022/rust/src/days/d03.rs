use std::{collections::HashSet, fs};

pub fn run(example: bool) {
    let path = "../input/03".to_string();
    let contents = fs::read_to_string(if example { path + "_example" } else { path }).unwrap();
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &str) -> i32 {
    contents
        .lines()
        .map(|l| -> i32 {
            let (left, right) = l.split_at(l.len() / 2);
            let left: HashSet<_> = left.chars().collect();
            let right: HashSet<_> = right.chars().collect();
            let common = left.intersection(&right).copied().next().unwrap();
            char_val(common)
        })
        .sum()
}

fn part_2(contents: &str) -> i32 {
    contents
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|elves| -> i32 {
            let mut elves = elves.iter();
            let first: HashSet<_> = elves.next().unwrap().chars().collect();
            let second: HashSet<_> = elves.next().unwrap().chars().collect();
            let third: HashSet<_> = elves.next().unwrap().chars().collect();
            let common: HashSet<_> = first.intersection(&second).copied().collect();
            let common = common.intersection(&third).copied().next().unwrap();
            char_val(common)
        })
        .sum()
}

fn char_val(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    }
}

#[cfg(test)]
mod tests {
    use super::{char_val, part_1, part_2};
    use std::fs;

    #[test]
    fn example() {
        let contents = fs::read_to_string("../input/03_example").unwrap();
        assert_eq!(part_1(&contents), 157);
        assert_eq!(part_2(&contents), 70);
    }

    #[test]
    fn char_val_edges() {
        assert_eq!(char_val('a'), 1);
        assert_eq!(char_val('z'), 26);
        assert_eq!(char_val('A'), 27);
        assert_eq!(char_val('Z'), 52);
    }
}
