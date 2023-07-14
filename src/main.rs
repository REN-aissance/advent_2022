use std::{env::args, process};

use aoc2022::{solver::Solve, *};

//Regex for counting lines of code: /^(?!^[ \)\};]*$)/gm
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Specify a puzzle number");
        process::exit(1);
    }

    let out = match &args[1].parse::<u8>().unwrap() {
        0 => panic!("Invalid puzzle number"),
        1 => day1::get_solver().solve(),
        2 => day2::get_solver().solve(),
        3 => day3::get_solver().solve(),
        4 => day4::get_solver().solve(),
        5 => day5::get_solver().solve(),
        6 => day6::get_solver().solve(),
        7 => day7::get_solver().solve(),
        8 => day8::get_solver().solve(),
        9 => day9::get_solver().solve(),
        10 => day10::get_solver().solve(),
        11 => day11::get_solver().solve(),
        12 => day12::get_solver().solve(),
        13.. => panic!("solution not yet implemented"),
    };
    println!("{:?}", out);
}
