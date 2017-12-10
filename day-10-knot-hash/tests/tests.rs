extern crate advent_of_code_2017_day_10;

use advent_of_code_2017_day_10::*;

#[test]
fn part_2_example_1() {
    assert_eq!(solve_puzzle_part_2(""), "a2582a3a0e66e6e86e3812dcb672a272");
}
#[test]
fn part_2_example_2() {
    assert_eq!(
        solve_puzzle_part_2("AoC 2017"),
        "33efeb34ea91902bb2f59c9920caa6cd"
    );
}
#[test]
fn part_2_example_3() {
    assert_eq!(
        solve_puzzle_part_2("1,2,3"),
        "3efbe78a8d82f29979031a4aa0b16a9d"
    );
}
#[test]
fn part_2_example_4() {
    assert_eq!(
        solve_puzzle_part_2("1,2,4"),
        "63960835bcdc130f0b66d7ff4f6a5a8e"
    );
}
