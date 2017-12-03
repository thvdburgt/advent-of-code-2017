pub fn solve_puzzle_part_1(input: &str) -> String {
    puzzle1(input).to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    puzzle2(input).to_string()
}

fn string_to_nums(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|string| string.parse::<u32>().unwrap())
        .collect()
}

fn input_to_nums(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(string_to_nums).collect()
}

fn puzzle1(input: &str) -> u32 {
    input_to_nums(input)
        .iter()
        .map(|nums| {
            let (max, min) = nums.iter().fold((0u32, std::u32::MAX), |(max, min), num| {
                (std::cmp::max(max, *num), std::cmp::min(min, *num))
            });
            max - min
        })
        .sum()
}

fn puzzle2(input: &str) -> u32 {
    input_to_nums(input)
        .iter()
        .filter_map(|nums| {
            for (i, n1) in nums.iter().enumerate() {
                for n2 in nums[(i + 1)..].iter() {
                    let (n1, n2) = if n1 > n2 { (n1, n2) } else { (n2, n1) };
                    if (n1 / n2) * n2 == *n1 {
                        return Some(n1 / n2);
                    }
                }
            }
            None
        })
        .sum()
}
