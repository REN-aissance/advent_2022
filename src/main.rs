use aoc2022::*;
use std::env::args;
use std::{fs, process};

//Regex for counting lines of code: /^(?!^[ \)\};]*$)/gm
fn main() {
    let args: Vec<String> = args().collect();
    let s;

    if args.len() != 2 && args.len() != 3 {
        eprintln!("Specifcy puzzle number and optionally \"test\"");
        process::exit(1);
    } else if args.get(2).is_some() {
        s = fs::read_to_string("./inputs/test.txt").unwrap();
    } else {
        s = fs::read_to_string("./inputs/".to_owned() + &args[1] + ".txt").unwrap_or_else(|_f| {
            eprintln!("Puzzle input text not found");
            process::exit(2);
        });
    }

    match &args[1].parse::<u8>().unwrap() {
        1 => day1::run(s),
        2 => day2::run(s),
        3 => day3::run(s),
        4 => day4::run(s),
        5 => day5::run(s),
        6 => day6::run(s),
        7 => day7::run(s),
        8 => day8::run(s),
        9 => day9::run(s),
        10 => day10::run(s),
        11 => day11::run(s),
        12 => day12::run(s),
        13 => day13::run(s),
        14 => day14::run(s),
        15 => day15::run(s),
        16 => day16::run(s),
        17 => day17::run(s),
        18 => day18::run(s),
        19 => day19::run(s),
        20 => day20::run(s),
        21 => day21::run(s),
        22 => day22::run(s),
        23 => day23::run(s),
        24 => day24::run(s),
        25 => day25::run(s),
        _ => panic!("invalid puzzle number"),
    }
}
