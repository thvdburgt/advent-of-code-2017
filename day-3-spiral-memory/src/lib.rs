mod spiral;

pub fn solve_puzzle_part_1(input: u32) -> String {
    //puzzle(input).to_string()
    puzzle1(input as usize).to_string()
}

pub fn solve_puzzle_part_2(input: u32) -> String {
    puzzle2(input).to_string()
}

fn puzzle1(index: usize) -> u32 {
    // loop over all rings, look for the location of index in the ring
    let square_location = spiral::SpiralRingIterator::new()
        .map(|ring| spiral::location_of_square_in_ring(&ring, index))
        .skip_while(|x| x.is_none())
        .next()
        .unwrap()
        .unwrap();

    // Manhattan distance of location
    (square_location.x.abs() + square_location.y.abs()) as u32
}


fn puzzle2(input: u32) -> u32 {
    // if the input is 0 then the answer is 1 in ring 0
    if input == 0 {
        return 1;
    }

    // we check every ring of the spiral for a number larger than the input.
    // we only need the previous ring to do this.
    let mut ring_iter = spiral::SpiralRingIterator::new();

    // create ring 0 and store 1 into it
    let mut prev_ring = ring_iter.next().unwrap();
    prev_ring.squares[0].content = Some(1);

    for ring in ring_iter {
        // create a mutable ring that we can store the values in
        let mut ring = ring.clone();

        // the loop for general rings doesn't work for ring 1
        if ring.n == 1 {
            //   5    4    2
            //  10    1    1
            //  11   23   25
            for i in 0..ring.squares.len() {
                let content: u32 = match i {
                    0 => 1,
                    1 => 2,
                    2 => 4,
                    3 => 5,
                    4 => 10,
                    5 => 11,
                    6 => 23,
                    7 => 25,
                    _ => panic!("invalid index in ring 1"),
                };
                if content > input {
                    return content;
                }
                ring.squares[i].content = Some(content);
            }
        } else {
            //  7   6   5   4   3
            //  8   3   2   1   2
            //  9   4   0   0   1
            // 10   5   6   7   0
            // 11  12  13  14  15
            let ring_length = ring.squares.len();
            let prev_ring_length = prev_ring.squares.len();

            let mut prev_val = 0;
            let mut prev_prev_val = 0;
            let mut corners_passed = 0;
            for i in 0..ring.squares.len() {
                // function to check if square i is a corner square
                let is_corner_square = |i| (i + 1) % (ring_length / 4) == 0;

                // j is the index of the square in the previous ring beside square i
                let j = (i + (ring_length - 1) + (ring_length - 2) * corners_passed) % ring_length;
                // f(i) returns the content of the square at index i in the previous ring
                let f = |i: usize| prev_ring.squares[i].content.unwrap();

                let content = if i == 0 {
                    // first square
                    f(0) + f(prev_ring_length - 1)
                } else if i == 1 {
                    // second square
                    f(0) + f(1) + f(prev_ring_length - 1) + prev_val
                } else if is_corner_square(i + 1) {
                    // one square before a corner
                    f(j - 1) + f(j) + prev_val + if i == ring_length - 2 {
                        // this is one square before last corner
                        ring.squares[0].content.unwrap()
                    } else {
                        0
                    }
                } else if is_corner_square(i) {
                    // corner square
                    corners_passed += 1;
                    f(j - 1) + prev_val + if i == ring_length - 1 {
                        // this is the last square
                        ring.squares[0].content.unwrap()
                    } else {
                        0
                    }
                } else if is_corner_square(i - 1) {
                    // one square after a corner
                    f(j) + f(j + 1) + prev_val + prev_prev_val
                } else {
                    // "normal" squares
                    f(j - 1) + f(j) + f(j + 1) + prev_val
                };

                // check if we found our answer
                if content > input {
                    return content;
                }

                // save value in square
                ring.squares[i].content = Some(content);
                // update values of previous squares
                prev_prev_val = prev_val;
                prev_val = content;
            }
        }

        // save copy of this ring for next iteration
        prev_ring = ring;
    }

    panic!("unable to solve puzzle 2")
}
