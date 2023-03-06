use std::vec::Vec;
use advent_lib::read::read_input;

fn part1(input: &Vec<String>) -> i64 {
    0
}

fn part2(input: &Vec<String>) -> i64 {
    0
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn dayNN_test() {
        let input: Vec<String> = test_input(include_str!("dayNN.testinput"));
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }
}
