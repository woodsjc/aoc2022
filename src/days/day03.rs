use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").expect("Unable to read day03.txt");
    let entries = input.split('\n').collect::<Vec<&str>>();

    let sol1: u64 = part1(&entries);
    let sol2: u64 = part2(&entries);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn part1(input: &Vec<&str>) -> u64 {
    let mut total = 0;
    for s in input {
        let m = s.len() / 2;
        let s = s.chars().collect::<Vec<char>>();
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();

        for i in 0..m {
            set1.insert(s[i]);
            set2.insert(s[i + m]);
        }

        //println!(
        //    "s:{:?}, m:{}, s1:{:?}, s2:{:?}, intersection:{:?}",
        //    s,
        //    m,
        //    set1,
        //    set2,
        //    set1.intersection(&set2)
        //);

        total += calc_priority(set1, set2);
    }
    total
}

fn part2(input: &Vec<&str>) -> u64 {
    let mut total = 0;
    let mut group = 0;
    let mut sets = [HashSet::new(), HashSet::new(), HashSet::new()];

    for s in input {
        let s = s.chars().collect::<Vec<char>>();

        if group >= sets.len() {
            group = 0;
            total += calc_priority(
                sets[0]
                    .intersection(&sets[1])
                    .cloned()
                    .collect::<HashSet<char>>(),
                sets[2].clone(),
            );
            sets = [HashSet::new(), HashSet::new(), HashSet::new()];
        }

        for c in s {
            sets[group].insert(c);
        }
        group += 1;
    }
    total
}

fn calc_priority(s1: HashSet<char>, s2: HashSet<char>) -> u64 {
    let mut total = 0u64;

    for b in s1.intersection(&s2).into_iter() {
        let mut to_add = 0;
        if b.is_ascii_lowercase() {
            to_add = (*b as u8 - 'a' as u8) as u64 + 1;
        } else if b.is_ascii_uppercase() {
            to_add = (*b as u8 - 'A' as u8) as u64 + 27;
        } else {
            println!("Invalid input: {b}");
        }
        //println!("b:{}, u8:{}, b-'a'+1:{}", *b, *b as u8, to_add);
        total += to_add;
    }

    total
}
