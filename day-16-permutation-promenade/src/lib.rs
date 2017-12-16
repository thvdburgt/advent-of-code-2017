use std::str::FromStr;

pub fn solve_puzzle_part_1(input: &str) -> String {
    let mut line: Vec<char> = "abcdefghijklmnop".chars().collect();
    let dance: Vec<_> = input.split(',').map(|m| m.parse().unwrap()).collect();

    exec_dance(&mut line, &dance);

    line.iter().collect()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    let orig_line: Vec<char> = "abcdefghijklmnop".chars().collect();

    let mut line = orig_line.clone();
    let dance: Vec<_> = input.split(',').map(|m| m.parse().unwrap()).collect();

    let mut cycle_size: usize = 0;
    loop {
        exec_dance(&mut line, &dance);
        cycle_size += 1;

        if line == orig_line {
            break;
        }
    }

    for _ in 0..(1_000_000_000 % cycle_size) {
        exec_dance(&mut line, &dance);
    }

    line.iter().collect()
}

fn exec_dance(line: &mut Vec<char>, dance: &[DanceMove]) {
    for dance_move in dance {
        dance_move.apply(line)
    }
}

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl DanceMove {
    fn apply(&self, line: &mut Vec<char>) {
        use DanceMove::*;

        match *self {
            Spin(x) => for _ in 0..x {
                let c = line.pop().unwrap();
                line.insert(0, c);
            },
            Exchange(a, b) => {
                line.swap(a, b);
            }
            Partner(a, b) => {
                let a_index = line.iter().position(|&x| x == a).unwrap();
                let b_index = line.iter().position(|&x| x == b).unwrap();
                line.swap(a_index, b_index);
            }
        }
    }
}

#[derive(Debug)]
struct DanceMoveParseError();

impl FromStr for DanceMove {
    type Err = DanceMoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DanceMove::*;
        let mut chars = s.chars();
        match chars.next() {
            Some('s') => {
                // sX
                let x = chars.as_str().parse().map_err(|_| DanceMoveParseError())?;
                Ok(Spin(x))
            }
            Some('x') => {
                // xA/B
                let mut positions = chars.as_str().split('/');
                let a = positions.next().ok_or(DanceMoveParseError())?;
                let a = a.parse().map_err(|_| DanceMoveParseError())?;
                let b = positions.next().ok_or(DanceMoveParseError())?;
                let b = b.parse().map_err(|_| DanceMoveParseError())?;
                Ok(Exchange(a, b))
            }
            Some('p') => {
                // pA/B
                let mut names = chars.as_str().split('/');
                let a = names.next().ok_or(DanceMoveParseError())?;
                let a = a.parse().map_err(|_| DanceMoveParseError())?;
                let b = names.next().ok_or(DanceMoveParseError())?;
                let b = b.parse().map_err(|_| DanceMoveParseError())?;
                Ok(Partner(a, b))
            }
            _ => Err(DanceMoveParseError()),
        }
    }
}

#[test]
fn part_1_example() {
    let mut line: Vec<char> = "abcde".chars().collect();
    let expected: Vec<char> = "baedc".chars().collect();
    let dance: Vec<_> = "s1,x3/4,pe/b"
        .split(',')
        .map(|m| m.parse().unwrap())
        .collect();

    exec_dance(&mut line, &dance);
    assert_eq!(line, expected);
}
