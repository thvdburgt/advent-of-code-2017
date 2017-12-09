extern crate advent_of_code_2017_day_9;

use advent_of_code_2017_day_9::*;

#[test]
fn part_1_example() {
    // {}, score of 1.
    assert_eq!(solve_puzzle_part_1("{}"), "1");
    // {{{}}}, score of 1 + 2 + 3 = 6.
    assert_eq!(solve_puzzle_part_1("{{{}}}"), "6");
    // {{},{}}, score of 1 + 2 + 2 = 5.
    assert_eq!(solve_puzzle_part_1("{{},{}}"), "5");
    // {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
    assert_eq!(solve_puzzle_part_1("{{{},{},{{}}}}"), "16");
    // {<a>,<a>,<a>,<a>}, score of 1.
    assert_eq!(solve_puzzle_part_1("{<a>,<a>,<a>,<a>}"), "1");
    // {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
    assert_eq!(solve_puzzle_part_1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), "9");
    // {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9."
    assert_eq!(solve_puzzle_part_1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), "9");
    // {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.
    assert_eq!(solve_puzzle_part_1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), "3");
}

#[test]
fn part_2_example() {
    // <>, 0 characters.
    assert_eq!(solve_puzzle_part_2("<>"), "0");
    // <random characters>, 17 characters.
    assert_eq!(solve_puzzle_part_2("<random characters>"), "17");
    // <<<<>, 3 characters.
    assert_eq!(solve_puzzle_part_2("<<<<>"), "3");
    // <{!>}>, 2 characters.
    assert_eq!(solve_puzzle_part_2("<{!>}>"), "2");
    // <!!>, 0 characters.
    assert_eq!(solve_puzzle_part_2("<!!>"), "0");
    // <!!!>>, 0 characters.
    assert_eq!(solve_puzzle_part_2("<!!!>>"), "0");
    // <{o"i!a,<{i<a>, 10 characters.
    assert_eq!(solve_puzzle_part_2("<{o\"i!a,<{i<a>"), "10");
}
