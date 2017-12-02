use std::fs::File;
use std::io::prelude::*;

extern crate inverse_captcha as advent;

fn main() {
    let mut file = File::open("input").expect("Could not open input");
    let mut input = String::new();
    file.read_to_string(&mut input).expect(
        "Could not read input",
    );

    let input = input.trim();
    let answer1 = advent::puzzle(input, 1);
    let answer2 = advent::puzzle(input, input.len() / 2);
    println!("the answer for part 1 is {}", answer1);
    println!("the answer for part 2 is {}", answer2);
}
