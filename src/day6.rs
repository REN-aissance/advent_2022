use std::collections::HashSet;

static PACKET_SIZE: usize = 14; // 4 for p1, 14 for p2

pub fn run(s: String) {
    let mut packet; //mutable to avoid repeated allocations
    for i in PACKET_SIZE..s.len() {
        unsafe {
            packet = s.get_unchecked((i - PACKET_SIZE)..i);
        }
        let num_uniques = packet.chars().collect::<HashSet<char>>().len();
        if num_uniques == PACKET_SIZE {
            println!("{}", i);
            break;
        }
    }
}
