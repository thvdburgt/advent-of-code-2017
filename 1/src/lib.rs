pub fn puzzle(input: &str, skip: usize) -> u32 {
    input
        .chars()
        .zip(input.chars().cycle().skip(skip))
        .filter_map(|(curr_char, next_char)| if curr_char == next_char {
            curr_char.to_digit(10)
        } else {
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit
        // and the third digit (2) matches the fourth digit.
        let input = "1122";
        assert_eq!(puzzle(input, 1), 3);
    }
    #[test]
    fn example2() {
        // 1111 produces 4 because each digit (all 1) matches the next.
        let input = "1111";
        assert_eq!(puzzle(input, 1), 4);
    }
    #[test]
    fn example3() {
        // 1234 produces 0 because no digit matches the next.
        let input = "1234";
        assert_eq!(puzzle(input, 1), 0);
    }
    #[test]
    fn example4() {
        // 91212129 produces 9 because the only digit that matches the next one is the last digit,
        // 9.
        let input = "91212129";
        assert_eq!(puzzle(input, 1), 9);
    }

    #[test]
    fn example5() {
        // 1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items
        // ahead.
        let input = "1212";
        assert_eq!(puzzle(input, input.len() / 2), 6);
    }

    #[test]
    fn example6() {
        // 1221 produces 0, because every comparison is between a 1 and a 2.
        let input = "1221";
        assert_eq!(puzzle(input, input.len() / 2), 0);
    }

    #[test]
    fn example7() {
        // 123425 produces 4, because both 2s match each other, but no other digit has a match.
        let input = "123425";
        assert_eq!(puzzle(input, input.len() / 2), 4);
    }

    #[test]
    fn example8() {
        // 123123 produces 12.
        let input = "123123";
        assert_eq!(puzzle(input, input.len() / 2), 12);
    }

    #[test]
    fn example9() {
        // 12131415 produces 4.
        let input = "12131415";
        assert_eq!(puzzle(input, input.len() / 2), 4);
    }
}
