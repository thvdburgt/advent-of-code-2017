extern crate advent_of_code_2017_day_13;

use advent_of_code_2017_day_13::*;

#[test]
fn part_1_example() {
    let input = "\
0: 3
1: 2
4: 4
6: 4";
    assert_eq!(solve_puzzle_part_1(input), "24");
}

#[test]
fn part_2_example() {
    let input = "\
0: 3
1: 2
4: 4
6: 4";
    assert_eq!(solve_puzzle_part_2(input), "10");
}
