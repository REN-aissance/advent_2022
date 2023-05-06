pub fn run(s: String) {
    let mut output1 = 0;
    let mut output2 = 0;
    for line in s.lines() {
        let ranges: Vec<Range> = line.split(',').map(Range::new).collect();
        if ranges[0].contains_or_contained(&ranges[1]) {
            output1 += 1;
        }
        if ranges[0].overlaps(&ranges[1]) {
            output2 += 1;
        }
    }
    println!("{}", output1);
    println!("{}", output2);
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
