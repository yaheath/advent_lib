use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

#[derive(Clone, Copy)]
enum Cell {
    Wall,
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Cell {
        match c {
            '#' => Cell::Wall,
            _ => Cell::Empty,
        }
    }
}

fn part1(input: &[String]) -> i64 {
    let grid: Grid<Cell> = Grid::from_input(input, Cell::Empty, 0);
    0
}

fn part2(input: &[String]) -> i64 {
    let grid: Grid<Cell> = Grid::from_input(input, Cell::Empty, 0);
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
    use ya_advent_lib::read::test_input;

    #[test]
    fn dayNN_test() {
        let input: Vec<String> = test_input(include_str!("dayNN.testinput"));
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }
}
