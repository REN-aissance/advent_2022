use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/<DAY>.txt", |_input| todo!())
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/<DAY>.txt";
    const EXPECTED_TEST: (&str, &str) = ("TODO", "TODO");
    const EXPECTED_REAL: (&str, &str) = ("TODO", "TODO");

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
