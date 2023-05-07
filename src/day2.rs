use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/2.txt", |input| {
        let mut output = vec![];
        let games: Vec<Vec<Hand>> = input
            .trim()
            .split('\n')
            .map(|f| {
                f.split(' ')
                    .map(|f| match f {
                        "A" => Hand::Rock,
                        "B" => Hand::Paper,
                        "C" => Hand::Scissors,
                        "X" => Hand::Rock,
                        "Y" => Hand::Paper,
                        "Z" => Hand::Scissors,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        let mut result = 0;
        for game in &games {
            result += game[1].eval(&game[0]);
        }
        output.push(result.to_string());

        result = 0;
        for mut game in games {
            match game[1] {
                Hand::Rock => game[1] = game[0].prev(),
                Hand::Paper => game[1] = game[0],
                Hand::Scissors => game[1] = game[0].next(),
            }
            result += game[1].eval(&game[0]);
        }
        output.push(result.to_string());
        output
    })
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    pub fn next(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
    pub fn prev(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
    pub fn value(&self) -> i32 {
        *self as i32
    }
    pub fn eval(self, rhs: &Hand) -> i32 {
        let mut result = 0;
        if self.eq(rhs) {
            result += 3; //draw
        } else if self == rhs.next() {
            result += 6; //win
        }
        result + self.value() //always add value of my hand
    }
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/2.txt";
    const EXPECTED_TEST: (&str, &str) = ("15", "12");
    const EXPECTED_REAL: (&str, &str) = ("15691", "12989");

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
