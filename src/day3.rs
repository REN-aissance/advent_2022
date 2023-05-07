use std::collections::{HashMap, HashSet};

use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/3.txt", |input| {
        let mut out = vec![];
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .zip(1..53);
        let a_table = HashMap::<char, u32>::from_iter(alphabet);

        let mut result = 0;
        for line in input.lines() {
            let (compartment_one, compartment_two) = line.split_at(line.len() / 2);
            let count_one = compartment_one.chars().collect::<HashSet<_>>().into_iter();
            let count_two = compartment_two.chars().collect::<HashSet<_>>();
            for c in count_one {
                if count_two.contains(&c) {
                    result += a_table.get(&c).unwrap();
                }
            }
        }
        out.push(result.to_string());

        let mut lines = input.lines().peekable();
        result = 0;
        while lines.peek().is_some() {
            let c1 = lines
                .next()
                .unwrap()
                .chars()
                .collect::<HashSet<_>>()
                .into_iter(); //super nice way to get unique chars
            let c2 = lines.next().unwrap().chars().collect::<HashSet<_>>();
            let c3 = lines.next().unwrap().chars().collect::<HashSet<_>>();
            for c in c1 {
                if c2.contains(&c) && c3.contains(&c) {
                    result += a_table.get(&c).unwrap();
                }
            }
        }
        out.push(result.to_string());
        out
    })
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/3.txt";
    const EXPECTED_TEST: (&str, &str) = ("157", "70");
    const EXPECTED_REAL: (&str, &str) = ("8185", "2817");

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
