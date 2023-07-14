use core::ops::{Add, AddAssign, Sub};
use std::collections::HashSet;

use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/9.txt", |input| {
        let mut rope1 = Rope::default();
        let mut rope2 = Rope::new(10);

        for line in input.lines() {
            let (delta, steps) = line.split_once(' ').unwrap();
            let steps = steps.parse::<u8>().unwrap();
            let delta = match delta {
                "R" => Vec2(1, 0),
                "L" => Vec2(-1, 0),
                "U" => Vec2(0, 1),
                "D" => Vec2(0, -1),
                _ => panic!("invalid direction {}", delta),
            };

            for _ in 0..steps {
                rope1.update(delta);
                rope2.update(delta);
            }
        }
        let o1 = rope1.visited.len().to_string();
        let o2 = rope2.visited.len().to_string();
        vec![o1, o2]
    })
}

struct Rope {
    segments: Vec<Vec2>,
    visited: HashSet<Vec2>,
}

impl Rope {
    fn default() -> Rope {
        let mut visited = HashSet::new();
        visited.insert(Vec2::default());
        let segments = vec![Vec2::default(); 2];
        Rope { segments, visited }
    }

    fn new(i: usize) -> Rope {
        let mut visited = HashSet::new();
        visited.insert(Vec2::default());
        let segments = vec![Vec2::default(); i];
        Rope { segments, visited }
    }

    fn update(&mut self, delta: Vec2) {
        for i in 0..self.segments.len() {
            if i == 0 {
                self.segments[i] += delta;
            } else {
                let prev = self.segments[i - 1];
                let mut dist = prev - self.segments[i];
                if dist.needs_normalization() {
                    dist.normalize();
                    self.segments[i] += dist;
                    if i == self.segments.len() - 1 {
                        self.visited.insert(self.segments[i]);
                    }
                } else {
                    break;
                }
            }
        }
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vec2(i32, i32);

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        Vec2(x, y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        Vec2(x, y)
    }
}

impl Vec2 {
    fn needs_normalization(&self) -> bool {
        self.0.abs() * self.1.abs() != 1 //excludes FIRST order diagonals
        && self.taxicab() > 1 //excludes orthoganally adjacent and center
    }
    fn taxicab(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
    fn normalize(&mut self) {
        if self.0 != 0 {
            self.0 /= self.0.abs();
        }
        if self.1 != 0 {
            self.1 /= self.1.abs();
        }
    }
}

#[cfg(test)]
pub mod test {

    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/9.txt";
    const EXPECTED_TEST: (&str, &str) = ("13", "1");
    const EXPECTED_REAL: (&str, &str) = ("5695", "2434");

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

    // #[ignore]
    // #[test]
    // fn move_up() {
    //     let mut r = Rope::new(10);
    //     for _ in 0..7 {
    //         r.update(Vec2(1, 0));
    //         r.update(Vec2(0, 1));
    //     }
    //     println!("{:?}", r.segments);
    //     for i in 0..=7 {
    //         assert!(r.segments.contains(&Vec2(i, i)));
    //     }
    // }

    // #[ignore]
    // #[test]
    // fn knights_move() {
    //     let mut r = Rope::new(2);
    //     r.update(Vec2(1, 0));
    //     r.update(Vec2(0, 1));
    //     r.update(Vec2(1, 0));
    //     println!("{:?}", r.segments);
    //     assert_eq!(r.visited.len(), 2);
    // }
}
