extern crate advent_of_code_2017_day_11;

use advent_of_code_2017_day_11::*;

#[test]
fn part_1_example_1() {
    assert_eq!(solve_puzzle_part_1("ne,ne,ne"), "3");
}

#[test]
fn part_1_example_2() {
    assert_eq!(solve_puzzle_part_1("ne,ne,sw,sw"), "0");
}

#[test]
fn part_1_example_3() {
    assert_eq!(solve_puzzle_part_1("ne,ne,s,s"), "2");
}

#[test]
fn part_1_example_4() {
    assert_eq!(solve_puzzle_part_1("se,sw,se,sw,sw"), "3");
}
