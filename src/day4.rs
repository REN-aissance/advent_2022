use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/4.txt", |input| {
        let mut output1 = 0;
        let mut output2 = 0;
        for line in input.lines() {
            let ranges: Vec<Range> = line.split(',').map(Range::new).collect();
            if ranges[0].contains_or_contained(&ranges[1]) {
                output1 += 1;
            }
            if ranges[0].overlaps(&ranges[1]) {
                output2 += 1;
            }
        }
        Vec::from([output1.to_string(), output2.to_string()])
    })
}

struct Range {
    pub lb: i32,
    pub ub: i32,
}

impl Range {
    fn new(s: &str) -> Range {
        let mut s = s.split('-');
        Range {
            lb: s.next().unwrap().parse().unwrap(),
            ub: s.next().unwrap().parse().unwrap(),
        }
    }
    fn contains(&self, rhs: &Range) -> bool {
        self.lb <= rhs.lb && self.ub >= rhs.ub
    }
    fn contains_or_contained(&self, rhs: &Range) -> bool {
        self.contains(rhs) || rhs.contains(self)
    }
    fn intersects(&self, rhs: &Range) -> bool {
        (self.lb <= rhs.lb && rhs.lb <= self.ub) || (self.lb <= rhs.ub && rhs.ub <= self.ub)
    }
    fn overlaps(&self, rhs: &Range) -> bool {
        self.intersects(rhs) || rhs.intersects(self)
    }
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/4.txt";
    const EXPECTED_TEST: (&str, &str) = ("2", "4");
    const EXPECTED_REAL: (&str, &str) = ("513", "878");

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
