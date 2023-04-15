use aoc2022::*;
use std::fs;

fn main() {
    let f = fs::read_to_string("./inputs/3.txt")
        .unwrap()
        .trim()
        .to_string();
    day3::run(f);
}
