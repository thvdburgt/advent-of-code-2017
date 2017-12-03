pub fn solve_puzzle_part1(input: &str) -> String {
    puzzle(input, 1).to_string()
}

pub fn solve_puzzle_part2(input: &str) -> String {
    puzzle(input, input.len() / 2).to_string()
}

pub fn puzzle(input: &str, skip: usize) -> u32 {
    input
        .chars()
        .zip(input.chars().cycle().skip(skip))
        .filter_map(|(curr_char, next_char)| if curr_char == next_char {
            curr_char.to_digit(10)
        } else {
            None
        })
        .sum()
}
