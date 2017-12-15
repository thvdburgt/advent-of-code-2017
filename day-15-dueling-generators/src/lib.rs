pub fn solve_puzzle_part_1(gen_a_start: u32, gen_b_start: u32) -> String {
    let gen_a = Generator::new(gen_a_start, 16_807);
    let gen_b = Generator::new(gen_b_start, 48_271);

    gen_a
        .zip(gen_b)
        .take(40_000_000)
        .filter(|&(v_a, v_b)| v_a & 0xFFFF == v_b & 0xFFFF)
        .count()
        .to_string()
}

pub fn solve_puzzle_part_2(gen_a_start: u32, gen_b_start: u32) -> String {
    let gen_a = Generator::new(gen_a_start, 16_807).filter(|n| n % 4 == 0);
    let gen_b = Generator::new(gen_b_start, 48_271).filter(|n| n % 8 == 0);

    gen_a
        .zip(gen_b)
        .take(5_000_000)
        .filter(|&(v_a, v_b)| v_a & 0xFFFF == v_b & 0xFFFF)
        .count()
        .to_string()
}

struct Generator {
    previous: u32,
    factor: u32,
}

impl Generator {
    fn new(start: u32, factor: u32) -> Generator {
        Generator {
            previous: start,
            factor,
        }
    }
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous = ((u64::from(self.previous) * u64::from(self.factor)) % 2_147_483_647) as u32;
        Some(self.previous)
    }
}
