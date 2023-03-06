use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use advent_lib::read::read_input;

struct Input {
    name: String,
    weight: i64,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\w+) (\d+)"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let name:String = caps.get(1).unwrap().as_str().into();
            let weight:i64 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Input {name, weight})
        }
        else {
            Err(())
        }
    }
}

fn part1(input: &Vec<Input>) -> usize {
    0
}

fn part2(input: &Vec<Input>) -> usize {
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
