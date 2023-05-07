use std::collections::HashSet;

use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/6.txt", |input| {
        vec![check_packet(input, 4), check_packet(input, 14)]
    })
}

fn check_packet(input: &str, packet_size: usize) -> String {
    input
        .as_bytes()
        .windows(packet_size)
        .position(|window| {
            return window.iter().collect::<HashSet<_>>().len() == packet_size;
        })
        .map(|x| x + packet_size)
        .unwrap()
        .to_string()
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/6.txt";
    const EXPECTED_TEST: (&str, &str) = ("7", "19");
    const EXPECTED_REAL: (&str, &str) = ("1343", "2193");

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
