use std::collections::{HashMap, HashSet};

pub fn run(s: String) {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .zip(1..53);
    let a_table = HashMap::<char, u32>::from_iter(alphabet);

    let mut result = 0;
    for line in s.lines() {
        let (compartment_one, compartment_two) = line.split_at(line.len() / 2);
        let count_one = compartment_one.chars().collect::<HashSet<_>>().into_iter();
        let count_two = compartment_two.chars().collect::<HashSet<_>>();
        for c in count_one {
            if count_two.contains(&c) {
                result += a_table.get(&c).unwrap();
            }
        }
    }
    println!("{}", result);

    let mut lines = s.lines().peekable();
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
    println!("{}", result);
}
