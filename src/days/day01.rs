use crate::{Solution, SolutionPair};
use std::collections::BinaryHeap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").expect("Unable to read day01.txt");
    let entries = input.split('\n').collect::<Vec<&str>>();
    let mut heap = BinaryHeap::new();
    let mut current = 0;
    let mut i = 0;

    //println!("Top 10 entries: \n{:?}", &entries[0..50]);

    while i < entries.len() {
        if entries[i] == "" {
            heap.push(current);
            current = 0;
        } else {
            current += entries[i]
                .parse::<i32>()
                .expect(&format!("Unable to parse {}", entries[i])) as i32;
        }
        i += 1;
    }

    let sol1: u64 = *heap.peek().expect("Empty heap") as u64;

    let mut tmp = 0;
    for _ in 0..3 {
        tmp += heap.pop().expect("Under 3 items in heap");
    }
    let sol2: u64 = tmp as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
