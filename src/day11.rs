use std::{collections::BinaryHeap, fmt::Display};

use regex::Regex;

use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/11.txt", |input| {
        let mut monkeys = make_monkeys(input, true);

        for _ in 0..20 {
            monkeys.update(true);
        }

        let mut mh = monkeys.contents.into_iter().collect::<BinaryHeap<Monkey>>();
        let m1 = mh.pop().unwrap();
        let m2 = mh.pop().unwrap();
        let mb1 = m1.monkey_business * m2.monkey_business;
        println!("{}", mb1);

        let mut monkeys = make_monkeys(input, false);

        for _ in 0..10_000 {
            monkeys.update(false);
        }

        let mut mh = monkeys.contents.into_iter().collect::<BinaryHeap<Monkey>>();
        let m1 = mh.pop().unwrap();
        let m2 = mh.pop().unwrap();
        let mb2 = m1.monkey_business * m2.monkey_business;
        println!("{}", mb2);

        vec![mb1.to_string(), mb2.to_string()]
    })
}

fn make_monkeys(input: &str, p1: bool) -> Monkeys {
    let items_re = Regex::new(r"Starting items: (.+)").expect("items regex");
    let op_re = Regex::new(r"old (.+)").expect("operations regex");
    let test_re = Regex::new(r"divisible by (.+)").expect("test regex");
    let true_re = Regex::new(r"If true: throw to monkey (.+)").expect("true regex");
    let false_re = Regex::new(r"If false: throw to monkey (.+)").expect("false regex");
    let items_list = flatten(items_re, input);
    let op_list = flatten(op_re, input);
    let test_list = flatten(test_re, input);
    let true_list = flatten(true_re, input);
    let false_list = flatten(false_re, input);
    let mut monkeys = Monkeys::new();
    for i in 0..items_list.len() {
        let items = items_list[i]
            .split(", ")
            .map(|s| s.parse::<_>().unwrap())
            .collect::<Vec<_>>();
        let op = op_list[i].split_once(' ').unwrap();
        let op = (op.0.to_owned(), op.1.to_owned());
        let test_val = test_list[i].parse::<_>().unwrap();
        let true_dest = true_list[i].parse::<usize>().unwrap();
        let false_dest = false_list[i].parse::<usize>().unwrap();

        monkeys.push(Monkey::new(items, op, test_val, true_dest, false_dest), p1);
    }
    monkeys
}

fn flatten(re: Regex, input: &str) -> Vec<String> {
    re.captures_iter(input)
        .map(|cap| cap[1].to_owned())
        .collect::<Vec<String>>()
}

struct Monkeys {
    supermod: Item,
    contents: Vec<Monkey>,
}

impl Monkeys {
    fn update(&mut self, p1: bool) {
        for i in 0..self.contents.len() {
            self.contents[i].update_items(p1);
            let items = self.contents[i].throw_items();
            for (val, dest) in items {
                let val = if !p1 { val % self.supermod } else { val };
                self.contents[dest].add_item(val);
            }
        }

        // print!("{}", self)
    }

    fn push(&mut self, m: Monkey, p1: bool) {
        if !p1 {
            self.supermod *= m.test_val;
        }
        self.contents.push(m);
    }

    fn new() -> Monkeys {
        Monkeys {
            supermod: 1,
            contents: vec![],
        }
    }
}

impl Display for Monkeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .contents
            .iter()
            .fold("".to_string(), |acc, m| acc + m.to_string().as_str() + "\n")
            .trim_end()
            .to_string();
        writeln!(f, "{}\n", &s)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Monkey {
    monkey_business: usize,
    items: Vec<Item>,
    op: (String, String),
    test_val: Item,
    true_dest: usize,
    false_dest: usize,
}

type Item = u64;

impl Monkey {
    fn new(
        items: Vec<Item>,
        op: (String, String),
        test_val: Item,
        true_dest: usize,
        false_dest: usize,
    ) -> Monkey {
        Monkey {
            monkey_business: 0,
            items,
            test_val,
            op,
            true_dest,
            false_dest,
        }
    }

    fn update_items(&mut self, p1: bool) {
        for item_val in self.items.iter_mut() {
            let e = if self.op.1.as_str() == "old" {
                item_val.to_owned()
            } else {
                self.op.1.parse::<_>().unwrap()
            };

            match self.op.0.as_str() {
                "*" => *item_val *= e,
                "+" => *item_val += e,
                _ => panic!("invalid operation"),
            }

            if p1 {
                *item_val /= 3;
            }

            self.monkey_business += 1;
        }
    }

    fn throw_items(&mut self) -> Vec<(Item, usize)> {
        let mut out = vec![];
        for item in self.items.iter() {
            if *item % self.test_val == 0 {
                out.push((*item, self.true_dest))
            } else {
                out.push((*item, self.false_dest))
            }
        }
        self.items.clear();
        out
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/11.txt";
    const EXPECTED_TEST: (&str, &str) = ("10605", "2713310158");
    const EXPECTED_REAL: (&str, &str) = ("55930", "14636993466");

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
