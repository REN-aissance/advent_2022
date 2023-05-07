use crate::solver::Solver;

/*In the future may be better to find an external library for
matricies or something that allows easy rotating of 2D vectors*/
#[allow(clippy::needless_range_loop)]
pub fn get_solver() -> Solver {
    Solver::new("./inputs/8.txt", |input| {
        let s = input.trim();

        let mut trees = init_grid(s);
        let height = trees.len();
        let width = trees[0].len();

        // Evaluate +X -X direction
        for y in 0..height {
            let mut ray1 = i8::MIN;
            let mut ray2 = i8::MIN;
            for x in 0..width {
                do_shadow(&mut trees[y][x], &mut ray1);
                do_shadow(&mut trees[y][width - x - 1], &mut ray2);
            }
        }
        // Evaluate +Y -Y direction
        for x in 0..width {
            let mut ray1 = i8::MIN;
            let mut ray2 = i8::MIN;
            for y in 0..height {
                do_shadow(&mut trees[y][x], &mut ray1);
                do_shadow(&mut trees[height - y - 1][x], &mut ray2);
            }
        }

        let mut max_view_score = 0;
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut left = 0;
                let mut right = 0;
                let mut up = 0;
                let mut down = 0;
                let tree = &trees[y][x];
                for i in (0..x).rev() {
                    left += 1;
                    if trees[y][i].height >= tree.height {
                        break;
                    }
                }
                for i in (x + 1)..height {
                    right += 1;
                    if trees[y][i].height >= tree.height {
                        break;
                    }
                }
                for i in (0..y).rev() {
                    up += 1;
                    if trees[i][x].height >= tree.height {
                        break;
                    }
                }
                for i in (y + 1)..height {
                    down += 1;
                    if trees[i][x].height >= tree.height {
                        break;
                    }
                }
                let view_score = left * right * up * down;
                if view_score > max_view_score {
                    max_view_score = view_score;
                }
            }
        }
        let trees_visible = get_trees_visible(&trees);

        println!("Best view: {} trees\n", max_view_score);
        vec![trees_visible, max_view_score.to_string()]
    })
}

fn get_trees_visible(trees: &[Vec<Tree>]) -> String {
    let mut trees_visible = 0;
    trees.iter().for_each(|row| {
        row.iter().for_each(|tree| match tree.visible {
            true => {
                trees_visible += 1;
                print!("1");
            }
            false => print!("0"),
        });
        println!();
    });
    println!("\nTrees visible: {}", trees_visible);
    trees_visible.to_string()
}

fn do_shadow(tree: &mut Tree, ray1: &mut i8) {
    if tree.height > *ray1 {
        tree.visible = true;
        *ray1 = tree.height;
    }
}

fn init_grid(s: &str) -> Vec<Vec<Tree>> {
    let mut trees: Vec<Vec<Tree>> = vec![];
    for line in s.lines() {
        let row: Vec<Tree> = line
            .trim()
            .chars()
            .map(|c| Tree {
                visible: false,
                height: c.to_digit(10).unwrap() as i8,
            })
            .collect();
        trees.push(row);
    }
    trees
}

#[derive(Debug)]
struct Tree {
    visible: bool,
    height: i8,
}

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/8.txt";
    const EXPECTED_TEST: (&str, &str) = ("21", "8");
    const EXPECTED_REAL: (&str, &str) = ("1733", "284648");

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
