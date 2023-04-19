use aoc2022::*;
use std::fs;

//Regex for counting lines of code: /^(?!^[ \)\};]*$)/gm
fn main() {
    let s = fs::read_to_string("./inputs/7.txt").unwrap();
    day7::run(s);
}
