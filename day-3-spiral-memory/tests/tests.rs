extern crate advent_of_code_2017_day_3;

use advent_of_code_2017_day_3::*;

#[test]
fn part_1_example_1() {
    // Data from square 1 is carried 0 steps, since it's at the access port.
    assert_eq!(solve_puzzle_part_1(1), "0");
}
#[test]
fn part_1_example_2() {
    // Data from square 12 is carried 3 steps, such as: down, left, left.
    assert_eq!(solve_puzzle_part_1(12), "3");
}
#[test]
fn part_1_example_3() {
    // Data from square 23 is carried only 2 steps: up twice.
    assert_eq!(solve_puzzle_part_1(23), "2");
}
#[test]
fn part_1_example_4() {
    // Data from square 1024 must be carried 31 steps.
    assert_eq!(solve_puzzle_part_1(1024), "31");
}

#[test]
fn part_2_examples() {
    // 147  142  133  122   59
    // 304    5    4    2   57
    // 330   10    1    1   54
    // 351   11   23   25   26
    // 362  747  806--->   ...
    let vals = vec![
        1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747,
        806,
    ];

    for i in 0..805 {
        for v in &vals {
            if i < *v {
                let answer = solve_puzzle_part_2(i);
                println!("i: {} -> {}", i, answer);
                assert_eq!(answer, v.to_string());
                break;
            }
        }
    }
}
