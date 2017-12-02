use std::fs::File;
use std::io::prelude::*;

extern crate corruption_checksum as advent;

fn main() {
    let mut file = File::open("input").expect("Could not open input");
    let mut input = String::new();
    file.read_to_string(&mut input).expect(
        "Could not read input",
    );

    let input = input.trim();
    let answer1 = advent::puzzle1(input);
    let answer2 = advent::puzzle2(input);
    println!("the answer for part 1 is {}", answer1);
    println!("the answer for part 2 is {}", answer2);
}
