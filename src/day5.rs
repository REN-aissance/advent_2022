use std::fmt::Display;

use regex::Regex;

use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/5.txt", |input| vec![p1(input), p2(input)])
}

fn p1(input: &str) -> String {
    let split = input.trim_end().split("\n\n").collect::<Vec<_>>();
    let lanes = populate_lanes(&split);
    let instructions = split[1].lines();

    for lane in &lanes {
        for c in lane {
            print!("{}", c);
        }
        println!()
    }

    let mut lanes = lanes;
    for line in instructions {
        let re = Regex::new(r#"(\d+)"#).expect("regex");
        let line = line.trim();
        let mut cap = re
            .captures_iter(line)
            .map(|x| x[0].to_string().parse::<usize>().unwrap());

        let count = cap.next().unwrap();
        let from = cap.next().unwrap() - 1;
        let to = cap.next().unwrap() - 1;
        for _ in 0..count {
            let c: Crate = lanes.get_mut(from).unwrap().pop().unwrap();
            lanes.get_mut(to).unwrap().push(c);
        }
    }

    lanes
        .into_iter()
        .map(|mut lane| lane.pop().unwrap().char)
        .collect::<String>()
}

fn p2(input: &str) -> String {
    let split = input.trim_end().split("\n\n").collect::<Vec<_>>();
    let mut lanes = populate_lanes(&split);
    let instructions = split[1].lines();

    for line in instructions {
        let re = Regex::new(r#"(\d+)"#).expect("regex");
        let line = line.trim();
        let mut cap = re
            .captures_iter(line)
            .map(|x| x[0].to_string().parse::<usize>().unwrap());

        let count = cap.next().unwrap();
        let from = cap.next().unwrap() - 1;
        let to = cap.next().unwrap() - 1;

        let mut clawstack: Vec<Crate> = Vec::new();
        for _ in 0..count {
            clawstack.insert(0, lanes.get_mut(from).unwrap().pop().unwrap());
        }
        lanes.get_mut(to).unwrap().append(&mut clawstack);
    }

    lanes
        .into_iter()
        .map(|mut lane| lane.pop().unwrap().char)
        .collect::<String>()
}

fn populate_lanes(split: &[&str]) -> Vec<Vec<Crate>> {
    let crate_string = split[0].rsplit_once('\n').unwrap().0;
    //discard last line
    let mut cratemap_iter = crate_string.lines().peekable();
    let mut lanes = build_lanes(cratemap_iter.peek().unwrap());
    {
        let lanes: &mut [Vec<Crate>] = &mut lanes;
        let mut cratemap_iter = cratemap_iter.rev().peekable();
        while cratemap_iter.peek().is_some() {
            let mut count = 0;
            let s = cratemap_iter.next().unwrap().to_string() + " ";
            let mut chars = s.chars().peekable();
            while chars.peek().is_some() {
                chars.next().unwrap(); //bracket
                let c = chars.next().unwrap();
                if c != ' ' {
                    lanes.get_mut(count).unwrap().push(Crate::new(c));
                }
                chars.next().unwrap(); //bracket
                chars.next().unwrap(); //spacer
                count += 1;
            }
        }
    };
    lanes
}

fn build_lanes(s: &str) -> Vec<Vec<Crate>> {
    let numlanes = (s.len() + 1) / 4;
    let mut lanes: Vec<Vec<Crate>> = Vec::new();
    for _ in 0..numlanes {
        lanes.push(Vec::new());
    }
    lanes
}

#[derive(Debug)]
struct Crate {
    pub char: char,
}

impl Crate {
    fn new(c: char) -> Self {
        Crate { char: c }
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char)
    }
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/5.txt";
    const EXPECTED_TEST: (&str, &str) = ("CMZ", "MCD");
    const EXPECTED_REAL: (&str, &str) = ("RFFFWBPNS", "CQQBBJFCS");

    #[test]
    fn test() {
        let solver = get_solver();
        if let Err(e) = solver.validate_on(TEST_PATH, EXPECTED_TEST) {
            panic!("{:?}", e);
        }
    }

    #[test]
    fn real() {
        let solver = get_solver();
        if let Err(e) = solver.validate(EXPECTED_REAL) {
            panic!("{:?}", e);
        }
    }
}
