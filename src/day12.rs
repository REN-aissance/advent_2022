use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
    time::Instant,
};

use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/12.txt", |input| {
        // println!("{}", input);

        let mut field = Field::new(input);
        field.solve();
        let path = field.path_length_from_start;
        // print_map(input, &field);
        let scenic = field.get_scenic_route();
        print_map(input, &field);

        vec![path.to_string(), scenic.to_string()]
    })
}

fn print_map(input: &str, field: &Field) {
    let temp = input.lines().collect::<Vec<_>>();
    let height = temp.len();
    let width = temp[0].len();
    let mut map = vec![vec!['0'; width]; height];
    for ((x, y), node) in &field.contents {
        map[*y][*x] = node.borrow().char;
    }
    for line in map {
        for char in line {
            print!("{}", char);
        }
        println!();
    }
}

type Node = Rc<RefCell<Tile>>;
type Pos = (usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    min_dist: u32,
    char: char,
    tiletype: TileType,
    pos: Pos,
    prev: Option<Node>,
    // heuristic: usize,
}

impl Tile {
    fn new(mut char: char, pos: Pos) -> Tile {
        let mut min_dist = u32::MAX;
        let mut tiletype = TileType::Normal;
        match char {
            'S' => {
                tiletype = TileType::Start;
                char = 'a';
            }
            'E' => {
                tiletype = TileType::End;
                char = 'z';
                min_dist = 0;
            }
            _ => (),
        }
        Tile {
            tiletype,
            char,
            min_dist,
            pos,
            prev: None,
            // heuristic: usize::MAX,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TileType {
    Start,
    Normal,
    End,
}

struct Field {
    start: Node,
    _end: Node,
    contents: HashMap<Pos, Node>,
    path_length_from_start: u32,
}

impl Field {
    fn new(input: &str) -> Field {
        let mut start = Rc::new(RefCell::new(Tile::new('E', (0, 0))));
        let mut end = Rc::new(RefCell::new(Tile::new('E', (0, 0))));
        let mut contents = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let node = Tile::new(char, (x, y));
                let node = Rc::new(RefCell::new(node));
                match node.borrow().tiletype {
                    TileType::Start => start = node.clone(),
                    TileType::End => end = node.clone(),
                    _ => (),
                }
                contents.insert((x, y), node);
            }
        }

        Field {
            contents,
            start,
            _end: end,
            path_length_from_start: u32::MAX,
        }
    }

    fn get_neighbors(&self, node: Node) -> Vec<Node> {
        let (x, y) = node.borrow().pos;
        let mut neighbors = vec![];
        let c = self.contents.get(&(x, y)).unwrap().borrow().char;

        //orthogonal neighbors
        if let Some(i) = y.checked_sub(1) {
            neighbors.push((x, i));
        }
        if let Some(i) = x.checked_add(1) {
            neighbors.push((i, y));
        }
        if let Some(i) = y.checked_add(1) {
            neighbors.push((x, i));
        }
        if let Some(i) = x.checked_sub(1) {
            neighbors.push((i, y));
        }

        let neighbors = neighbors
            .iter()
            .filter_map(|pos| {
                let node = self.contents.get(pos)?;

                //represents the climbing rules
                match c as i8 - node.borrow().char as i8 {
                    2.. => None,
                    _ => Some(node.clone()),
                }
            })
            .collect();

        neighbors
    }

    fn solve(&mut self) {
        let now = Instant::now();
        //Dijkstra's
        let mut visited: HashSet<Pos> = HashSet::new();
        let mut node_mpq = self.contents.values().cloned().collect::<Vec<Node>>();
        node_mpq.sort();
        while let Some(node) = node_mpq.iter().find(|n| !visited.contains(&n.borrow().pos)) {
            visited.insert(node.borrow().pos);
            let dist = node.borrow().min_dist.saturating_add(1);
            for neighbor in self.get_neighbors(node.clone()) {
                let ndist = neighbor.clone().borrow().min_dist;
                if dist < ndist {
                    (*neighbor).borrow_mut().min_dist = dist;
                    (*neighbor).borrow_mut().prev = Some(node.clone());
                }
            }
            node_mpq.sort();
        }
        self.path_length_from_start = self.find_end_dist();
        println!("Done! Elapsed: {} ms", now.elapsed().as_millis());
        self.mark_path(self.start.clone(), ' '); //Lazily reuses the char space of tiles, field cannot be reused.
    }

    fn mark_path(&self, node: Node, char: char) {
        let mut cur = node;
        (*cur).borrow_mut().char = char;
        while let Some(node) = cur.clone().borrow().prev.clone() {
            node.borrow_mut().char = char;
            cur = node.clone();
        }
    }

    fn get_scenic_route(&self) -> u32 {
        let mut contents = self
            .contents
            .values()
            .filter(|node| matches!(node.borrow().char, 'a'))
            .collect::<Vec<_>>();

        contents.sort();
        let node = contents[0];
        self.mark_path(node.clone(), '.');
        node.borrow().min_dist
    }

    fn find_end_dist(&self) -> u32 {
        self.start.borrow().min_dist
    }
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/12.txt";
    const EXPECTED_TEST: (&str, &str) = ("31", "29");
    const EXPECTED_REAL: (&str, &str) = ("420", "414");

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
