pub fn solve_puzzle_part_1(input: &str) -> String {
    input
        .lines()
        .map(is_valid_passphrase_part_1)
        .filter(|&bool| bool)
        .count()
        .to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    input
        .lines()
        .map(is_valid_passphrase_part_2)
        .filter(|&bool| bool)
        .count()
        .to_string()
}

fn is_valid_passphrase_part_1(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();

    // go over all pairs of words looking for duplicates
    for (i, word1) in words.iter().enumerate() {
        for word2 in words[i + 1..].iter() {
            if word1 == word2 {
                return false;
            }
        }
    }

    // no duplicates found
    true
}

fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut s1_chars: Vec<_> = s1.chars().collect();
    let mut s2_chars: Vec<_> = s2.chars().collect();
    s1_chars.sort();
    s2_chars.sort();
    s1_chars == s2_chars
}

fn is_valid_passphrase_part_2(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();

    // go over all pairs of words looking for anagrams
    for (i, word1) in words.iter().enumerate() {
        for word2 in words[i + 1..].iter() {
            if are_anagrams(word1, word2) {
                return false;
            }
        }
    }

    // no anagrams found
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_examples() {
        assert!(is_valid_passphrase_part_1("aa bb cc dd ee"));
        assert!(is_valid_passphrase_part_1("aa bb cc dd aa") == false);
        assert!(is_valid_passphrase_part_1("aa bb cc dd aaa"));
    }

    #[test]
    fn part_2_examples() {
        assert!(is_valid_passphrase_part_2("abcde fghij"));
        assert!(is_valid_passphrase_part_2("abcde xyz ecdab") == false);

        assert!(is_valid_passphrase_part_2("a ab abc abd abf abj"));
        assert!(is_valid_passphrase_part_2("iiii oiii ooii oooi oooo"));
        assert!(is_valid_passphrase_part_2("oiii ioii iioi iiio") == false);
    }
}
