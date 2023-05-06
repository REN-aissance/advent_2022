use super::PuzzleError;
use super::*;
use std::collections::BinaryHeap;

const PATH: &str = "./inputs/1.txt";

pub fn run() -> Result<Vec<String>, PuzzleError> {
    run_on(PATH)
}

fn run_on(path: &str) -> Result<Vec<String>, PuzzleError> {
    let input = get_input(path);
    let mut bheap = BinaryHeap::<u32>::new();
    let mut output: Vec<String> = vec![];

    //This is probably not best practice, but I am practicing iterators.
    input
        .trim()
        .split("\n\n")
        .map(|f| {
            f.trim()
                .split('\n')
                .map(|f| f.parse::<u32>().unwrap())
                .reduce(|acc, x| acc + x)
                .unwrap()
        })
        .for_each(|f| bheap.push(f));

    let mut x = bheap.pop().unwrap();
    output.push(x.to_string());
    x += bheap.pop().unwrap() + bheap.pop().unwrap();
    output.push(x.to_string());
    Ok(output)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn day1_test() {
        let result = Vec::from(["24000", "45000"]);
        assert_eq!(run_on("./test_inputs/1.txt").unwrap(), result);
    }

    #[test]
    fn day1() {
        let result = Vec::from(["67016", "200116"]);
        assert_eq!(run().unwrap(), result);
    }
}
