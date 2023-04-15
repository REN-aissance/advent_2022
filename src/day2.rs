pub fn run(s: String) {
    let games: Vec<Vec<Hand>> = s
        .trim()
        .split("\n")
        .map(|f| {
            f.split(" ")
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
    println!("{}", result);

    result = 0;
    for mut game in games {
        match game[1] {
            Hand::Rock => game[1] = game[0].prev(),
            Hand::Paper => game[1] = game[0],
            Hand::Scissors => game[1] = game[0].next(),
        }
        result += game[1].eval(&game[0]);
    }
    println!("{}", result);
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
        return *self as i32;
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
