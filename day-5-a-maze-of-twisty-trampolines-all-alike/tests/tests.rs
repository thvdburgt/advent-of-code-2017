extern crate advent_of_code_2017_day_5;

use advent_of_code_2017_day_5::*;


static EXAMPLE_INPUT: &'static str = "0\n3\n0\n1\n-3\n";

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(EXAMPLE_INPUT), "5");
}

#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2(EXAMPLE_INPUT), "10");
}
