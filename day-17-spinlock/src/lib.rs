pub fn solve_puzzle_part_1(steps: usize) -> u32 {
    let mut buffer = vec![0];
    let mut position = 0;

    for i in 0..2017 {
        position = 1 + (position + steps) % (i as usize + 1);
        buffer.insert(position, i + 1);
    }

    buffer[(position + 1) % buffer.len()]
}

pub fn solve_puzzle_part_2(steps: usize) -> u32 {
    let mut position = 0;
    let mut val_after_zero = 0;

    for i in 0..50_000_000 {
        position = 1 + (position + steps) % (i as usize + 1);
        if position == 1 {
            val_after_zero = i + 1
        }
    }

    val_after_zero
}
