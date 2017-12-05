extern crate advent_of_code_2017_day_2;

use advent_of_code_2017_day_2::*;

#[test]
fn part_1_example() {
    // 5 1 9 5
    // 7 5 3
    // 2 4 6 8
    //
    // - The first row's largest and smallest values are 9 and 1, and their difference is 8.
    // - The second row's largest and smallest values are 7 and 3, and their difference is 4.
    // - The third row's difference is 6.
    //
    // In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
    let input = "\
5 1 9 5
7 5 3
2 4 6 8";
    println!("{}", input);
    assert_eq!(solve_puzzle_part_1(input), "18");
}

#[test]
fn part_2_example() {
    // 5 9 2 8
    // 9 4 7 3
    // 3 8 6 5

    // - In the first row, the only two numbers that evenly divide are 8 and 2; the result of
    //   this division is 4.
    // - In the second row, the two numbers are 9 and 3; the result is 3.
    // - In the third row, the result is 2.
    //
    // In this example, the sum of the results would be 4 + 3 + 2 = 9.
    let input = "\
5 9 2 8
9 4 7 3
3 8 6 5";
    println!("{}", input);
    assert_eq!(solve_puzzle_part_2(input), "9");
}
