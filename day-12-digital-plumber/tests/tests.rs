extern crate advent_of_code_2017_day_12;

use advent_of_code_2017_day_12::*;

#[test]
fn part_1_example() {
    let input = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    assert_eq!(solve_puzzle_part_1(input), "6");
}

#[test]
fn part_2_example() {
    let input = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    assert_eq!(solve_puzzle_part_2(input), "2");
}
