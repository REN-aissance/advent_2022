use std::fs;

type Solution = Box<dyn Fn(&str) -> SolveResult>;
type SolveResult = Vec<String>;
type PuzzleResult = Result<(), PuzzleError>;

#[derive(Debug)]
pub enum PuzzleError {
    Failure,
    PartialSuccess,
}

pub struct Solver {
    input: String,
    solution: Solution,
}

impl Solver {
    pub fn new(path: &str, solution: impl Fn(&str) -> SolveResult + 'static) -> Solver {
        Solver {
            input: Self::get_input(path),
            solution: Box::new(solution),
        }
    }
}

impl Solve for Solver {
    fn solve(&self) -> SolveResult {
        (*self.solution)(&self.input)
    }

    fn solve_on(&self, path: &str) -> SolveResult {
        let input = Self::get_input(path);
        (*self.solution)(&input)
    }
}

impl Validate for Solver {
    fn validate(&self, expected: (&str, &str)) -> PuzzleResult {
        let result = self.solve();
        get_result(result, expected)
    }

    fn validate_on(&self, path: &str, expected: (&str, &str)) -> PuzzleResult {
        let result = self.solve_on(path);
        get_result(result, expected)
    }
}

fn get_result(result: Vec<String>, expected: (&str, &str)) -> Result<(), PuzzleError> {
    if result[0] != expected.0 {
        return Err(PuzzleError::Failure);
    }
    if result[1] != expected.1 {
        return Err(PuzzleError::PartialSuccess);
    }
    Ok(())
}

pub trait Solve {
    fn get_input(path: &str) -> String {
        fs::read_to_string(path).unwrap_or_else(|err| {
            panic!("{}", err);
        })
    }
    fn solve_on(&self, path: &str) -> SolveResult;
    fn solve(&self) -> SolveResult;
}

pub trait Validate {
    fn validate(&self, expected: (&str, &str)) -> PuzzleResult;
    fn validate_on(&self, path: &str, expected: (&str, &str)) -> PuzzleResult;
}
