extern crate advent_of_code_2017_day_1;

use advent_of_code_2017_day_1::*;

#[test]
fn part_1_example_1() {
    // 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit
    // and the third digit (2) matches the fourth digit.
    let input = "1122";
    assert_eq!(solve_puzzle_part_1(input), "3");
}
#[test]
fn part_1_example_2() {
    // 1111 produces 4 because each digit (all 1) matches the next.
    let input = "1111";
    assert_eq!(solve_puzzle_part_1(input), "4");
}
#[test]
fn part_1_example_3() {
    // 1234 produces 0 because no digit matches the next.
    let input = "1234";
    assert_eq!(solve_puzzle_part_1(input), "0");
}
#[test]
fn part_1_example_4() {
    // 91212129 produces 9 because the only digit that matches the next one is the last digit,
    // 9.
    let input = "91212129";
    assert_eq!(solve_puzzle_part_1(input), "9");
}

#[test]
fn part_2_example_1() {
    // 1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items
    // ahead.
    let input = "1212";
    assert_eq!(solve_puzzle_part_2(input), "6");
}

#[test]
fn part_2_example_2() {
    // 1221 produces 0, because every comparison is between a 1 and a 2.
    let input = "1221";
    assert_eq!(solve_puzzle_part_2(input), "0");
}

#[test]
fn part_2_example_3() {
    // 123425 produces 4, because both 2s match each other, but no other digit has a match.
    let input = "123425";
    assert_eq!(solve_puzzle_part_2(input), "4");
}

#[test]
fn part_2_example_4() {
    // 123123 produces 12.
    let input = "123123";
    assert_eq!(solve_puzzle_part_2(input), "12");
}

#[test]
fn part_2_example_5() {
    // 12131415 produces 4.
    let input = "12131415";
    assert_eq!(solve_puzzle_part_2(input), "4");
}
