#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_string();
        for _ in 0..5 {
            s = s.replace("  ", " ");
        }

        fn parse_size(s: &str) -> Result<usize, Error> {
            assert!(&s[s.len() - 1..] == "T");
            Ok(s[..s.len() - 1].parse()?)
        }

        let parts = s.split(" ").collect::<Vec<&str>>();
        let name = parts[0].split("-").collect::<Vec<&str>>();
        let x = name[1][1..].parse()?;
        let y = name[2][1..].parse()?;
        let size = parse_size(parts[1])?;
        let used = parse_size(parts[2])?;
        let avail = parse_size(parts[3])?;
        Ok(Self {
            x,
            y,
            size,
            used,
            avail,
        })
    }
}

#[derive(Default)]
pub struct Day22 {
    nodes: HashMap<(usize, usize), Node>,
    goal: (usize, usize),
    empty: (usize, usize),
}

impl Day22 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_shortest_path(
        &self,
        empty: (usize, usize),
        target: (usize, usize),
        ignore: (usize, usize),
    ) -> usize {
        let mut seen = Vec::new();
        let mut queue = Vec::new();
        seen.push(empty);
        queue.push((0, empty));

        let mut i = 0;
        while i < queue.len() {
            let steps = queue[i].0;
            let coord = queue[i].1;

            for dir in 0..4 {
                let ncoords = match dir {
                    0 => {
                        if coord.0 == 0 {
                            continue;
                        } else {
                            (coord.0 - 1, coord.1)
                        }
                    }
                    1 => (coord.0 + 1, coord.1),
                    2 => {
                        if coord.1 == 0 {
                            continue;
                        } else {
                            (coord.0, coord.1 - 1)
                        }
                    }
                    3 => (coord.0, coord.1 + 1),
                    _ => unreachable!(),
                };

                if ncoords.0 == ignore.0 && ncoords.1 == ignore.1 {
                    continue;
                }
                if seen.contains(&ncoords) {
                    continue;
                }

                let nused = match self.nodes.get(&ncoords) {
                    Some(neighbor) => neighbor.used,
                    None => continue,
                };

                let csize = match self.nodes.get(&coord) {
                    Some(cur) => cur.size,
                    None => unreachable!(),
                };

                if nused > csize {
                    continue;
                }

                if ncoords.0 == target.0 && ncoords.1 == target.1 {
                    return steps + 1;
                }

                seen.push(ncoords);
                queue.push((steps + 1, ncoords));
            }

            i += 1;
        }

        panic!("Cannot find path from {:?} to {:?}", empty, target);
    }
}

impl Runner for Day22 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if line.starts_with("/dev/grid/") {
                let node: Node = line.parse()?;
                self.goal.0 = self.goal.0.max(node.x);
                if node.used == 0 {
                    self.empty = (node.x, node.y);
                }
                self.nodes.insert((node.x, node.y), node);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut viable = 0;
        for a in self.nodes.values() {
            for b in self.nodes.values() {
                if a == b || a.used == 0 {
                    continue;
                }
                if a.used <= b.avail {
                    viable += 1;
                }
            }
        }
        Ok(viable.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut goal = self.goal;
        let mut empty = self.empty;
        let mut target = (goal.0 - 1, goal.1);

        let mut dist = self.find_shortest_path(empty, target, goal);
        dist += 1;
        goal = target;
        while goal.0 != 0 || goal.1 != 0 {
            empty = (goal.0 + 1, 0);
            target = (goal.0 - 1, 0);
            dist += self.find_shortest_path(empty, target, goal);
            dist += 1;
            goal = target;
        }
        Ok(dist.into())
    }
}
