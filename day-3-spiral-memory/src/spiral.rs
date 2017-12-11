use std::cmp;

#[derive(Debug, PartialEq)]
pub struct SquareLocation {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Square {
    pub index: usize,
    pub content: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Ring {
    pub n: u32,
    pub squares: Vec<Square>,
}

#[derive(Debug)]
pub struct SpiralRingIterator {
    curr_ring: Ring,
    next_ring_n: u32,
}

impl SpiralRingIterator {
    pub fn new() -> SpiralRingIterator {
        let curr_ring = Ring {
            n: 0,
            squares: vec![
                Square {
                    index: 1,
                    content: None,
                },
            ],
        };
        SpiralRingIterator {
            curr_ring,
            next_ring_n: 0,
        }
    }
}

impl Iterator for SpiralRingIterator {
    type Item = Ring;

    fn next(&mut self) -> Option<Ring> {
        // check if this is the first ring
        if self.next_ring_n == 0 {
            // FIXME: This feels dirty
            self.next_ring_n += 1;
            return Some(self.curr_ring.clone());
        }

        // information on the current ring
        let last_index_curr = self.curr_ring.squares.last().unwrap().index;

        // create next ring
        let first_index_next = last_index_curr + 1;
        let length_next = 8 * (self.curr_ring.n as usize + 1);
        let next_squares = (first_index_next..)
            .take(length_next as usize)
            .map(|index| Square {
                index,
                content: None,
            })
            .collect();
        let next_ring = Ring {
            n: self.next_ring_n,
            squares: next_squares,
        };

        // increase ring n
        self.next_ring_n += 1;
        self.curr_ring = next_ring.clone();

        Some(next_ring)
    }
}

pub fn location_of_square_in_ring(ring: &Ring, square_index: usize) -> Option<SquareLocation> {
    if !(ring.squares.first().unwrap().index <= square_index
        && square_index <= ring.squares.last().unwrap().index)
    {
        return None;
    }

    // start with location of first square in this ring:
    let location_first_square = SquareLocation {
        x: ring.n as i32,
        y: if ring.n == 0 { 0 } else { 1 - (ring.n as i32) },
    };

    // step trough the ring until the square
    let spiral_size = 2 * ring.n + 1;
    assert!(spiral_size > 0);

    let mut steps_to_take: u32 = square_index as u32 - ring.squares.first().unwrap().index as u32;
    let mut location_square = location_first_square;

    // folow spiral right hand side up
    let steps = cmp::min(spiral_size.saturating_sub(2), steps_to_take);
    location_square.y += steps as i32;
    steps_to_take -= steps;

    // folow spiral top left
    let steps = cmp::min(spiral_size - 1, steps_to_take);
    location_square.x -= steps as i32;
    steps_to_take -= steps;

    // folow spiral left hand side down
    let steps = cmp::min(spiral_size - 1, steps_to_take);
    location_square.y -= steps as i32;
    steps_to_take -= steps;

    // folow spiral top left
    let steps = cmp::min(spiral_size - 1, steps_to_take);
    location_square.x += steps as i32;
    steps_to_take -= steps;

    assert!(steps_to_take == 0);

    Some(location_square)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_of_square_in_ring() {
        let mut ring_iter = SpiralRingIterator::new();
        let ring = ring_iter.next().unwrap();
        assert_eq!(
            location_of_square_in_ring(&ring, 1),
            Some(SquareLocation { x: 0, y: 0 })
        );
        let ring = ring_iter.next().unwrap();
        assert_eq!(
            location_of_square_in_ring(&ring, 2),
            Some(SquareLocation { x: 1, y: 0 })
        );
        assert_eq!(
            location_of_square_in_ring(&ring, 3),
            Some(SquareLocation { x: 1, y: 1 })
        );
        assert_eq!(
            location_of_square_in_ring(&ring, 4),
            Some(SquareLocation { x: 0, y: 1 })
        );
    }
}
