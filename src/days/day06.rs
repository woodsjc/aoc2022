use crate::{Solution, SolutionPair};
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06.txt").expect("Unable to read day06.txt");
    //let entries = input.split('\n').collect::<Vec<&str>>();
    let mut prior = VecDeque::new();
    let mut prior_part2 = VecDeque::new();
    let mut count = 0;
    let mut part1_complete = false;
    let mut part2_complete = false;
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    for c in input.chars() {
        count += 1;
        if part1_complete && part2_complete {
            break;
        }
        if !part1_complete {
            if prior.len() < 4 {
                prior.push_back(c);
            } else {
                prior.pop_front();
                prior.push_back(c);
                let unique: HashSet<&char> = HashSet::from_iter(prior.iter());
                if unique.len() == prior.len() {
                    println!("prior:{:?}, c:{c}, count:{count}", prior);
                    part1_complete = true;
                    sol1 = count;
                }
            }
        }

        if !part2_complete {
            if prior_part2.len() < 14 {
                prior_part2.push_back(c);
            } else {
                prior_part2.pop_front();
                prior_part2.push_back(c);
                let unique: HashSet<&char> = HashSet::from_iter(prior_part2.iter());
                if unique.len() == prior_part2.len() {
                    println!("prior:{:?}, c:{c}, count:{count}", prior_part2);
                    part2_complete = true;
                    sol2 = count;
                }
            }
        }
    }

    (Solution::U64(sol1), Solution::U64(sol2))
}
