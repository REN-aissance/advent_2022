use regex::Regex;

pub fn run(s: String) {
    let mut s_iter = s.lines();
    let cratemap = build_cratemap(&mut s_iter);
    let mut cratemap_iter = cratemap.lines().peekable();
    let mut lanes = build_lanes(cratemap_iter.next().unwrap());

    populate_lanes(cratemap_iter, &mut lanes);

    for line in s_iter {
        let re = Regex::new(r#"(\d+)"#).expect("regex");
        let line = line.trim();
        let mut cap = re
            .captures_iter(line)
            .map(|x| x[0].to_string().parse::<usize>().unwrap());

        let count = cap.next().unwrap();
        let from = cap.next().unwrap() - 1;
        let to = cap.next().unwrap() - 1;

        let mut clawstack: Vec<Crate> = Vec::new();
        for _ in 0..count {
            clawstack.insert(0, lanes.get_mut(from).unwrap().pop().unwrap());

            // Part 1 solution
            // let c: Crate = lanes.get_mut(from).unwrap().pop().unwrap();
            // lanes.get_mut(to).unwrap().push(c);
        }
        lanes.get_mut(to).unwrap().append(&mut clawstack);
    }

    for lane in &mut lanes {
        print!("{}", lane.pop().unwrap().char);
    }
    println!();
}

fn build_cratemap(s_iter: &mut std::str::Lines) -> String {
    let mut cratemap = String::new();
    for _ in 0..9 {
        cratemap = s_iter.next().unwrap().to_string() + "\n" + &cratemap;
    }
    s_iter.next();
    cratemap
}

fn populate_lanes(
    mut cratemap_iter: std::iter::Peekable<std::str::Lines>,
    lanes: &mut [Vec<Crate>],
) {
    while cratemap_iter.peek().is_some() {
        let mut count = 0;
        let s = cratemap_iter.next().unwrap().to_string() + " ";
        let mut chars = s.chars().peekable();
        while chars.peek().is_some() {
            chars.next().unwrap(); //bracket
            let c = chars.next().unwrap();
            if c != ' ' {
                lanes.get_mut(count).unwrap().push(Crate::new(c));
            }
            chars.next().unwrap(); //bracket
            chars.next().unwrap(); //spacer
            count += 1;
        }
    }
}

fn build_lanes(s: &str) -> Vec<Vec<Crate>> {
    let numlanes = (s.len() + 1) / 4;
    let mut lanes: Vec<Vec<Crate>> = Vec::new();
    for _ in 0..numlanes {
        lanes.push(Vec::new());
    }
    lanes
}

#[derive(Debug)]
struct Crate {
    pub char: char,
}

impl Crate {
    fn new(c: char) -> Self {
        Crate { char: c }
    }
}
