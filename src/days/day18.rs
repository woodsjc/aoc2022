use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn neighbors(self) -> [Cube; 6] {
        [
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        ]
    }

    fn within(self, min: i32, max: i32) -> bool {
        self.x >= min
            && self.x <= max
            && self.y >= min
            && self.y <= max
            && self.z >= min
            && self.z <= max
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day18.txt").expect("Unable to read day18.txt");
    let entries = input.split('\n').collect::<Vec<&str>>();

    let mut faces = 0;
    let mut cubes = HashSet::new();

    for e in entries {
        if e.len() < 5 {
            break;
        }
        let e = e
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        //println!("e:{:?}", e);

        let current = Cube {
            x: e[0],
            y: e[1],
            z: e[2],
        };
        cubes.insert(current);
        faces += 6;
        for c in current.neighbors() {
            if cubes.contains(&c) {
                faces -= 2;
            }
        }
    }

    let mut stack = vec![Cube { x: 0, y: 0, z: 0 }];
    let mut seen = HashSet::new();
    let max = cubes.iter().map(|x| x.x.max(x.y.max(x.z))).max().unwrap();
    while let Some(q) = stack.pop() {
        for c in q.neighbors() {
            if cubes.contains(&c) || seen.contains(&c) || !c.within(-1, max + 1) {
                continue;
            }
            seen.insert(c);
            stack.push(c);
        }
    }

    let sol1: u64 = faces as u64;
    let sol2: u64 = cubes
        .into_iter()
        .flat_map(|c| c.neighbors())
        .filter(|c| seen.contains(c))
        .count() as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
