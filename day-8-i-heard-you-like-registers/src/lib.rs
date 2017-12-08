use std::collections::HashMap;
use std::str::FromStr;

pub fn solve_puzzle_part_1(input: &str) -> String {
    puzzle_1(input).0.to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    puzzle_1(input).1.to_string()
}

fn puzzle_1(input: &str) -> (i32, i32) {
    let mut registers = HashMap::new();
    let mut highest_during = 0;

    for line in input.lines() {
        let (op_str, rest) = split_n_words(line, 3).unwrap();
        let (if_str, cond_str) = rest.split_at(4); // TODO might panic
        assert_eq!(if_str, " if ");

        let op: Operation = op_str.parse().unwrap();
        let cond: Condition = cond_str.parse().unwrap();

        if cond.test(&mut registers) {
            op.apply(&mut registers, &mut highest_during);
        }
    }

    (*registers.values().max().unwrap(), highest_during)
}

fn split_n_words(s: &str, mut n: u32) -> Option<(&str, &str)> {
    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            n -= 1;
        }
        if n == 0 {
            return Some(s.split_at(i));
        }
    }
    None
}

#[derive(Debug, Clone)]
enum Condition {
    Lt(String, i32),
    LtEq(String, i32),
    Eq(String, i32),
    NotEq(String, i32),
    GtEq(String, i32),
    Gt(String, i32),
}

impl Condition {
    pub fn test(&self, regs: &mut HashMap<String, i32>) -> bool {
        use Condition::*;

        let reg = match *self {
            Lt(ref reg, _)
            | LtEq(ref reg, _)
            | Eq(ref reg, _)
            | NotEq(ref reg, _)
            | GtEq(ref reg, _)
            | Gt(ref reg, _) => reg,
        };

        let val = regs.entry(reg.to_string()).or_insert(0);

        match *self {
            Lt(_, n) => *val < n,
            LtEq(_, n) => *val <= n,
            Eq(_, n) => *val == n,
            NotEq(_, n) => *val != n,
            GtEq(_, n) => *val >= n,
            Gt(_, n) => *val > n,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseConditionError {
    kind: ConditionErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConditionErrorKind {
    Empty,
    NoCondition,
    NoConditionValue,
    InvalidCondition,
    InvalidConditionValue,
    TrailingWords,
}

impl FromStr for Condition {
    type Err = ConditionErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Condition::*;
        use ConditionErrorKind::*;

        let mut words = s.split_whitespace();

        let r = words.next().ok_or(Empty)?;
        let cond = words.next().ok_or(NoCondition)?;
        let cond_val = words.next().ok_or(NoConditionValue)?;
        if words.next().is_some() {
            return Err(TrailingWords);
        };

        let r = r.to_string();
        let cond_val = cond_val.parse().or(Err(InvalidConditionValue))?;
        match cond {
            "<" => Ok(Lt(r, cond_val)),
            "<=" => Ok(LtEq(r, cond_val)),
            "==" => Ok(Eq(r, cond_val)),
            "!=" => Ok(NotEq(r, cond_val)),
            ">=" => Ok(GtEq(r, cond_val)),
            ">" => Ok(Gt(r, cond_val)),
            _ => Err(InvalidCondition),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Inc(String, i32),
    Dec(String, i32),
}

impl Operation {
    pub fn apply(&self, regs: &mut HashMap<String, i32>, highest_during: &mut i32) {
        use Operation::*;

        let reg = match *self {
            Inc(ref reg, _) | Dec(ref reg, _) => reg,
        };

        let val = regs.entry(reg.to_string()).or_insert(0);

        *val += match *self {
            Inc(_, n) => n,
            Dec(_, n) => -n,
        };

        if *val > *highest_during {
            *highest_during = *val;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOperationError {
    kind: OperationErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OperationErrorKind {
    Empty,
    NoOperation,
    NoOperand,
    InvalidOperation,
    InvalidOperand,
    TrailingWords,
}

impl FromStr for Operation {
    type Err = OperationErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operation::*;
        use OperationErrorKind::*;

        let mut words = s.split_whitespace();

        let r = words.next().ok_or(Empty)?;
        let op = words.next().ok_or(NoOperation)?;
        let operand = words.next().ok_or(NoOperand)?;
        if words.next().is_some() {
            return Err(TrailingWords);
        };

        let r = r.to_string();
        let operand = operand.parse().or(Err(InvalidOperand))?;

        match op {
            "inc" => Ok(Inc(r, operand)),
            "dec" => Ok(Dec(r, operand)),
            _ => Err(InvalidOperation),
        }
    }
}
