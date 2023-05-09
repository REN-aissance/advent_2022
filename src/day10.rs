use crate::solver::Solver;

/*
   I am not sure how I feel about this solution. It would have been
   easier to compute changes for each pseudo-frame but I deliberately avoided
   it for the challenge.
*/

pub fn get_solver() -> Solver {
    Solver::new("./inputs/10.txt", |input| {
        let mut cpu = Cpu::new();
        let mut acc = 0;
        for line in input.lines() {
            if let Some((_, val)) = line.split_once(' ') {
                let val = val.parse::<i32>().unwrap();
                cpu.addx(val);
            } else {
                cpu.noop();
            }
            cpu.extract_desired(&mut acc);
            cpu.print_display_char();
        }
        println!();
        vec![acc.to_string(), String::new()]
    })
}

struct Cpu {
    x_old: i32,
    x: i32,
    clk_old: u32,
    clk: u32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            x: 1,
            x_old: 1,
            clk: 0,
            clk_old: 0,
        }
    }
    fn addx(&mut self, i: i32) {
        self.cache();
        self.clk += 2;
        self.x += i;
    }
    fn noop(&mut self) {
        self.cache();
        self.clk += 1;
    }
    fn cache(&mut self) {
        self.clk_old = self.clk;
        self.x_old = self.x;
    }
    fn extract_desired(&self, acc: &mut i32) {
        for i in (self.clk_old + 1)..=self.clk {
            // specific request by the puzzle 20, 60, 100, 140, 180, 220, etc
            if (i + 20) % 40 == 0 {
                let signal = i as i32 * self.x_old;
                // println!("cycle: {}, x: {}, signal: {}", i, self.x_old, signal);
                *acc += signal;
            }
        }
    }

    fn print_display_char(&self) {
        for i in self.clk_old..self.clk {
            let i = i % 40;
            if i == 0 {
                println!();
            }
            let sprite_pixels = match self.x_old {
                -1 => vec![0],
                0 => vec![0, 1],
                1..=38 => vec![self.x_old - 1, self.x_old, self.x_old + 1],
                39 => vec![38, 39],
                40 => vec![39],
                _ => panic!("Invalid pixel position {}", self.x_old),
            };
            //double pixels for readability
            if sprite_pixels.contains(&(i as i32)) {
                print!("@@");
            } else {
                print!("..");
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/10.txt";
    const EXPECTED_TEST: (&str, &str) = ("13140", "");
    const EXPECTED_REAL: (&str, &str) = ("12640", "");

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
        // EHBZLRJR
    }
}
