use std::fs;

pub fn run(example: bool) {
    let path = "../input/01".to_string();
    let contents = fs::read_to_string(if example { path + "_example" } else { path }).unwrap();
    let (part1, part2) = solve(&contents);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn solve(contents: &str) -> (i32, i32) {
    let mut sums = contents
        .split("\n\n")
        .map(|lines| {
            return lines
                .split("\n")
                .map(|x| return x.parse::<i32>().unwrap())
                .sum::<i32>();
        })
        .collect::<Vec<i32>>();

    let part1 = *sums.iter().max().unwrap();

    sums.sort_unstable_by(|a, b| b.cmp(a));
    let part2 = sums.iter().take(3).sum::<i32>();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs;

    #[test]
    fn example() {
        let contents = fs::read_to_string("../input/01_example").unwrap();
        let (part1, part2) = solve(&contents);
        assert_eq!(part1, 24000);
        assert_eq!(part2, 45000);
    }
}
