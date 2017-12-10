pub fn solve_puzzle_part_1(input: &str) -> String {
    let mut list: Vec<u32> = (0..256).collect();
    let lengths: Vec<usize> = input
        .trim()
        .split(',')
        .map(|e| e.parse().unwrap())
        .collect();
    let mut current_position = 0;
    let mut skip_size = 0;

    round(&mut list, &lengths, &mut current_position, &mut skip_size);
    (list[0] * list[1]).to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    let mut list: Vec<u32> = (0..256).collect();
    let lengths: Vec<usize> = input
        .bytes()
        .map(|b| b as usize)
        .chain(vec![17, 31, 73, 47, 23].into_iter())
        .collect();
    let mut current_position = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        round(&mut list, &lengths, &mut current_position, &mut skip_size);
    }

    list.chunks(16)
        .map(|block| block.iter().fold(0, |acc, &x| acc ^ x))
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<_>>()
        .join("")
}

fn round(
    list: &mut Vec<u32>,
    lengths: &[usize],
    current_position: &mut usize,
    skip_size: &mut usize,
) {
    let list_len = list.len();

    for length in lengths {
        // reverse section
        for i in 0..(*length / 2) {
            list.swap(
                (*current_position + i) % list_len,
                (*current_position + length - 1 - i) % list_len,
            );

        }

        // update current_position
        *current_position = (*current_position + *length + *skip_size) % list_len;
        // increase skip size
        *skip_size += 1;
    }
}

#[test]
fn example_part1() {
    let mut list = vec![0, 1, 2, 3, 4];
    let lengths = vec![3, 4, 1, 5];
    round(&mut list, &lengths, &mut 0, &mut 0);
    assert_eq!(list[0] * list[1], 12);
}
