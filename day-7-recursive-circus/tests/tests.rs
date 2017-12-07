extern crate advent_of_code_2017_day_7;

use advent_of_code_2017_day_7::*;

#[test]
fn part_1_example() {
    let input = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    assert_eq!(solve_puzzle_part_1(input), "tknk");
}

#[test]
fn part_2_example() {
    let input = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    assert_eq!(solve_puzzle_part_2(input), "60");
}
