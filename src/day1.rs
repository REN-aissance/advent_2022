use std::collections::BinaryHeap;

use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/1.txt", |input| {
        //This is probably not best practice, but I am practicing iterators.
        let mut elves = input
            .trim()
            .split("\n\n")
            .map(|f| {
                f.split('\n')
                    .map(|f| f.parse::<u32>().unwrap())
                    .reduce(|acc, x| acc + x)
                    .unwrap()
            })
            .collect::<BinaryHeap<u32>>();

        let p1 = elves.pop().unwrap();
        let p2 = p1 + elves.pop().unwrap() + elves.pop().unwrap();
        let p1 = p1.to_string();
        let p2 = p2.to_string();
        vec![p1, p2]
    })
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/1.txt";
    const EXPECTED_TEST: (&str, &str) = ("24000", "45000");
    const EXPECTED_REAL: (&str, &str) = ("67016", "200116");

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
