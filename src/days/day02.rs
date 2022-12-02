use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02.txt").expect("Unable to read day02.txt");
    let entries = input.split('\n').collect::<Vec<&str>>();
    //let mut strategy: HashMap<&str, &str> = HashMap::from([("A", "Y"), ("B", "X"), ("C", "Z")]);
    let winning_strategy: HashMap<&str, &str> = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);

    let mut total = 0;
    let mut part2_total = 0;
    for s in entries {
        if s == "" {
            break;
        }
        let mut play = s.split(' ');
        //println!("play: {:?}", play);
        let first = play.next().expect("Invalid play");
        let second = play.next().expect("Invalid play");

        if winning_strategy
            .get(first)
            .expect("Unable to find play in strategy")
            == &second
        {
            total += 6;
        } else if first == "A" && second == "X"
            || first == "B" && second == "Y"
            || first == "C" && second == "Z"
        {
            total += 3;
        }

        match second {
            "X" => {
                total += 1;

                //lose
                part2_total += 0;
                part2_total += match first {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => 0,
                };
            }
            "Y" => {
                total += 2;

                //draw
                part2_total += 3;
                part2_total += match first {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => 0,
                };
            }
            "Z" => {
                total += 3;

                //win
                part2_total += 6;
                part2_total += match first {
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => 0,
                };
            }
            _ => {}
        }
    }

    // Your solution here...
    let sol1: u64 = total as u64;
    let sol2: u64 = part2_total as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
