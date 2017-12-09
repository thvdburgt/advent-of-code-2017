pub fn solve_puzzle_part_1(input: &str) -> String {
    solve(input).0.to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    solve(input).1.to_string()
}

fn solve(input: &str) -> (u32, u32) {
    let mut in_garbage = false;
    let mut level = 0;
    let mut score = 0;
    let mut garbage_count = 0;

    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '!' => {
                chars.next();
            }
            '>' => in_garbage = false,
            _ if in_garbage => garbage_count += 1,
            '<' => in_garbage = true,
            '{' => level += 1,
            '}' => {
                score += level;
                level -= 1
            }
            ',' => {}
            _ => panic!("found {}", c),
        };
    }

    (score, garbage_count)
}
