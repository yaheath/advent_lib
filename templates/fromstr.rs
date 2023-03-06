use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

struct Input {
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input{})
    }
}

fn part1(input: &Vec<Input>) -> i64 {
    0
}

fn part2(input: &Vec<Input>) -> i64 {
    0
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn dayNN_test() {
        let input: Vec<Input> = test_input(include_str!("dayNN.testinput"));
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }
}
