extern crate advent_of_code_2017_day_15 as advent;

fn main() {
    let gen_a_start = 516;
    let gen_b_start = 190;
    println!(
        "the answer for part 1 is {}",
        advent::solve_puzzle_part_1(gen_a_start, gen_b_start)
    );
    println!(
        "the answer for part 2 is {}",
        advent::solve_puzzle_part_2(gen_a_start, gen_b_start)
    );
}
