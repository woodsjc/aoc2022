use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04.txt").expect("Unable to read day04.txt");
    let entries = input.split('\n').collect::<Vec<&str>>();
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    for line in entries.iter() {
        if line.is_empty() {
            break;
        }
        let l = line.split_once(",");
        let t1 = l.unwrap().0.split_once("-");
        let (start1, end1) = (
            t1.unwrap().0.parse::<i32>().unwrap(),
            t1.unwrap().1.parse::<i32>().unwrap(),
        );
        let t2 = l.unwrap().1.split_once("-");
        let (start2, end2) = (
            t2.unwrap().0.parse::<i32>().unwrap(),
            t2.unwrap().1.parse::<i32>().unwrap(),
        );

        if start1 >= start2 && end1 <= end2 || start2 >= start1 && end2 <= end1 {
            sol1 += 1;
            sol2 += 1;
        } else if start1 >= start2 && start1 <= end2
            || end1 >= start2 && end1 <= end2
            || start2 >= start1 && start2 <= end1
            || end2 >= start1 && end2 <= end1
        {
            //sol2 += (end1.min(end2) - start1.max(start2)) as u64;
            sol2 += 1;
        }
    }

    (Solution::U64(sol1), Solution::U64(sol2))
}
