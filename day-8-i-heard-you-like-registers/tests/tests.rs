extern crate advent_of_code_2017_day_8;

use advent_of_code_2017_day_8::*;

#[test]
fn part_1_example() {
    let input = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
    assert_eq!(solve_puzzle_part_1(input), "1");
}

#[test]
fn part_2_example() {
    let input = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
    assert_eq!(solve_puzzle_part_2(input), "10");
}
