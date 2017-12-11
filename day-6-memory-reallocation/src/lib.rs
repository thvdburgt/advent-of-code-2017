use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve_puzzle_part_1(input: &str) -> String {
    let mut banks = string_to_nums(input);

    let mut seen = HashSet::new();
    let mut cycle = 0;
    while !seen.contains(&banks) {
        seen.insert(banks.clone());
        cycle_bank(&mut banks);
        cycle += 1;
    }

    cycle.to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    let mut banks = string_to_nums(input);

    let mut seen = HashMap::new();
    let mut cycle: u32 = 0;
    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), cycle);
        cycle_bank(&mut banks);
        cycle += 1;
    }

    (cycle - seen[&banks]).to_string()
}

fn string_to_nums(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|string| string.parse::<u32>().unwrap())
        .collect()
}

fn cycle_bank(banks: &mut Vec<u32>) {
    // find the memory bank with the most blocks (use of negative index to make sure ties are won
    // by the lowest-numbered memory bank)
    let (max_i, &max_val) = banks
        .iter()
        .enumerate()
        .max_by_key(|&(i, x)| (x, -(i as isize)))
        .expect("empty list of banks");
    let banks_len = banks.len();

    banks[max_i] = 0;
    // add quotient to each bank
    let quotient = max_val / banks_len as u32;
    for x in banks.iter_mut() {
        *x += quotient;
    }
    // add 1 to ecah bank for 'remainder' banks from max bank
    let remainder = max_val % banks_len as u32;
    for i in (max_i + 1)..(max_i + 1 + remainder as usize) {
        banks[i % banks_len] += 1;
    }
}
