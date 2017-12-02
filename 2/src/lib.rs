fn to_nums(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|string| string.parse::<u32>().unwrap())
        .collect()
}

pub fn puzzle1(input: &str) -> u32 {
    input
        .lines()
        .map(to_nums)
        .map(|nums| {
            let (max, min) = nums.iter().fold((0u32, std::u32::MAX), |(max, min), num| {
                (std::cmp::max(max, *num), std::cmp::min(min, *num))
            });
            max - min
        })
        .sum()
}

pub fn puzzle2(input: &str) -> u32 {
    input
        .lines()
        .map(to_nums)
        .filter_map(|nums| {
            for (i, n1) in nums.iter().enumerate() {
                for n2 in nums[(i + 1)..].iter() {
                    let (n1, n2) = if n1 > n2 { (n1, n2) } else { (n2, n1) };
                    if (n1 / n2) * n2 == *n1 {
                        return Some(n1 / n2);
                    }
                }
            }
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // 5 1 9 5
        // 7 5 3
        // 2 4 6 8
        //
        // - The first row's largest and smallest values are 9 and 1, and their difference is 8.
        // - The second row's largest and smallest values are 7 and 3, and their difference is 4.
        // - The third row's difference is 6.
        //
        // In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
        let input = "\
5 1 9 5
7 5 3
2 4 6 8";
        println!("{}", input);
        assert_eq!(puzzle1(input), 18);
    }

    #[test]
    fn example2() {
        // 5 9 2 8
        // 9 4 7 3
        // 3 8 6 5

        // - In the first row, the only two numbers that evenly divide are 8 and 2; the result of
        //   this division is 4.
        // - In the second row, the two numbers are 9 and 3; the result is 3.
        // - In the third row, the result is 2.
        //
        // In this example, the sum of the results would be 4 + 3 + 2 = 9.
        let input = "\
5 9 2 8
9 4 7 3
3 8 6 5";
        println!("{}", input);
        assert_eq!(puzzle2(input), 9);
    }
}
