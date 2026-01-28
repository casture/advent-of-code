use std::fs;

pub fn run(example: bool) {
    let path = "../input/03".to_string();
    let contents = fs::read_to_string(if example { path + "_example" } else { path }).unwrap();
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &str) -> i32 {}

fn part_2(contents: &str) -> i32 {}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use std::fs;

    #[test]
    fn example() {
        let contents = fs::read_to_string("../input/03_example").unwrap();
        assert_eq!(part_1(&contents), 157);
        assert_eq!(part_2(&contents), );
    }
}
