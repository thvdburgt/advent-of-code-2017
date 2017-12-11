use std::str::FromStr;

pub fn solve_puzzle_part_1(input: &str) -> String {
    let path: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let origin = CubeCoordinate { x: 0, y: 0, z: 0 };
    let mut c = origin.clone();
    c.follow_path(path.as_slice());

    c.distance_to(&origin).to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    let origin = CubeCoordinate { x: 0, y: 0, z: 0 };
    let mut c = origin.clone();

    let mut max_distance = 0;

    for direction in input.trim().split(',').map(|s| s.parse().unwrap()) {
        c.step(&direction);
        max_distance = std::cmp::max(max_distance, c.distance_to(&origin));
    }

    max_distance.to_string()
}

#[derive(Debug, Clone)]
struct CubeCoordinate {
    //           #---#
    //          /y+1  \
    //     #---#       #---#
    //    /y+1  \     /     \
    //   #       #---#    x+1#
    //    \     / y   \     /
    //     #---#     x #---#
    //    /     \ z   /     \
    //   #       #---#    x+1#
    //    \z+1  /     \     /
    //     #---#       #---#
    //          \z+1  /
    //           #---#
    x: i32,
    y: i32,
    z: i32,
}

impl CubeCoordinate {
    fn step(&mut self, direction: &Direction) {
        use Direction::*;
        let (dx, dy, dz) = match *direction {
            North => (0, 1, -1),
            NorthEast => (1, 0, -1),
            SouthEast => (1, -1, 0),
            South => (0, -1, 1),
            SouthWest => (-1, 0, 1),
            NorthWest => (-1, 1, 0),
        };

        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    fn follow_path(&mut self, path: &[Direction]) {
        for direction in path {
            self.step(direction);
        }
    }

    fn distance_to(&self, other: &CubeCoordinate) -> u32 {
        ((self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
            + (self.z - other.z).abs() as u32) / 2
    }
}

#[derive(Debug, Clone)]
enum Direction {
    //   \  n  /
    // nw #---# ne
    //   /     \
    // -#       #-
    //   \     /
    // sw #---# se
    //   /  s  \
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseDirectionError {
    UnknownDirection,
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        use ParseDirectionError::*;

        match s {
            "n" => Ok(North),
            "ne" => Ok(NorthEast),
            "se" => Ok(SouthEast),
            "s" => Ok(South),
            "sw" => Ok(SouthWest),
            "nw" => Ok(NorthWest),
            _ => Err(UnknownDirection),
        }
    }
}
