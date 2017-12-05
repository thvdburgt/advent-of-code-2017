pub fn solve_puzzle_part_1(input: &str) -> String {
    let jumps = input
        .lines()
        .map(|string| string.parse::<i32>().unwrap())
        .collect();
    puzzle_1(jumps).to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    let jumps = input
        .lines()
        .map(|string| string.parse::<i32>().unwrap())
        .collect();
    puzzle_2(jumps).to_string()
}

fn puzzle_1(mut jumps: Vec<i32>) -> u32 {
    let mut curr_jump = 0;
    let mut steps = 0;
    loop {
        steps += 1;

        // get the offset
        let offset = jumps[curr_jump];

        // check if we escaped
        let next_jump: i32 = curr_jump as i32 + offset;
        if next_jump < 0 || next_jump >= jumps.len() as i32 {
            break;
        }

        // take the jump
        jumps[curr_jump] += 1;
        curr_jump = next_jump as usize;
    }

    steps
}

fn puzzle_2(mut jumps: Vec<i32>) -> u32 {
    let mut curr_jump = 0;
    let mut steps = 0;
    loop {
        steps += 1;

        // get the offset
        let offset = jumps[curr_jump];

        // check if we escaped
        let next_jump: i32 = curr_jump as i32 + offset;
        if next_jump < 0 || next_jump >= jumps.len() as i32 {
            break;
        }

        // take the jump
        jumps[curr_jump] += if offset >= 3 { -1 } else { 1 };
        curr_jump = next_jump as usize;
    }

    steps
}
