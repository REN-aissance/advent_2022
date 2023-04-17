use aoc2022::*;
use std::fs;

fn main() {
    let s = fs::read_to_string("./inputs/6.txt").unwrap();
    day6::run(s);
}
