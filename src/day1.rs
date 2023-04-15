use std::collections::BinaryHeap;

pub fn run(s: String) {
    let mut bheap = BinaryHeap::<u32>::new();

    //This is probably not best practice, but I am practicing iterators.
    s.split("\n\n")
        .map(|f| {
            f.trim()
                .split("\n")
                .map(|f| f.parse::<u32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .for_each(|f| bheap.push(f));

    let mut x = bheap.pop().unwrap();
    println!("{}", x);
    x += bheap.pop().unwrap() + bheap.pop().unwrap();
    println!("{}", x);
}
