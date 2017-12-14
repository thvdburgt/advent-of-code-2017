pub fn solve_puzzle_part_1(input: &str) -> String {
    let hw: u32 = (0..128)
        .map(|n| format!("{}-{}", input, n))
        .map(|s| knot_hash(s.as_bytes()))
        .map(|h| h.iter().map(|b| u32::from(hamming_weight(*b))).sum::<u32>())
        .sum();

    hw.to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    let mut disk: Vec<Vec<char>> = (0..128).map(|n| format!("{}-{}", input, n))
        // calculate the hash
        .map(|s| knot_hash(s.as_bytes()).iter()
             // create a string for each byte in the hash
             .map(|b| format!("{:08b}", b))
             // join all the strings into one single string for the hash
             .collect::<Vec<_>>()
             .join("")
             // use '.' and '#' instead of 0 and 1
             .chars().map(|c| {
                 match c {
                     '0' => '.',
                     '1' => '#',
                     _   => panic!(),
                 }
             }
             )
             .collect()
            )
        .collect();

    regions(&mut disk).to_string()
}

fn regions(disk: &mut Vec<Vec<char>>) -> u32 {
    fn dfs(disk: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
        // check if there is a region at (row, col), if there is remove it and
        // return true
        if disk[row][col] == '#' {
            // remove it
            disk[row][col] = 'X';
            // and the rest of the region
            if col > 0 {
                dfs(disk, row, col - 1);
            }
            if row > 0 {
                dfs(disk, row - 1, col);
            }
            if col < disk[row].len() - 1 {
                dfs(disk, row, col + 1);
            }
            if row < disk.len() - 1 {
                dfs(disk, row + 1, col);
            }
            true
        } else {
            false
        }
    }

    let mut count = 0;

    for row in 0..disk.len() {
        for col in 0..disk[0].len() {
            assert!(disk[row].len() == disk[0].len());
            if dfs(disk, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn hamming_weight(x: u8) -> u8 {
    let m1 = 0x55; // 01010101
    let m2 = 0x33; // 00110011
    let m4 = 0x0f; // 00001111

    let x = (x & m1) + ((x >> 1) & m1); // put count of each 2 bits into those 2 bits
    let x = (x & m2) + ((x >> 2) & m2); // put count of each 4 bits into those 4 bits

    // put count of each 8 bits into those 8 bits (and return it)
    (x & m4) + ((x >> 4) & m4)
}

fn knot_hash(input: &[u8]) -> Vec<u8> {
    let mut list: Vec<u8> = (0..256u32).map(|x| x as u8).collect();

    let lengths: Vec<usize> = input
        .iter()
        .map(|&b| b as usize)
        .chain(vec![17, 31, 73, 47, 23].into_iter())
        .collect();
    let mut current_position = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        round(&mut list, &lengths, &mut current_position, &mut skip_size);
    }

    list.chunks(16)
        .map(|block| block.iter().fold(0, |acc, &x| acc ^ x))
        .collect()
}

fn round(
    list: &mut Vec<u8>,
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
