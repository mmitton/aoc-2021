#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "/dev/grid/node-x{}-y{}\t{}T\t{}T\t{}T",
            self.x, self.y, self.size, self.used, self.avail
        )
    }
}

impl TryFrom<&str> for Node {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
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
            x: x,
            y: y,
            size: size,
            used: used,
            avail: avail,
        })
    }
}

fn load_input(
    filename: &str,
) -> Result<
    (
        BTreeMap<(usize, usize), Node>,
        (usize, usize),
        (usize, usize),
    ),
    Error,
> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut nodes = BTreeMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut empty = (0, 0);

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if line.starts_with("/dev/grid/") {
            let node: Node = line.try_into()?;
            if node.x > max_x {
                max_x = node.x;
            }
            if node.y > max_y {
                max_y = node.y;
            }
            if node.used == 0 {
                empty = (node.x, node.y);
            }
            nodes.insert((node.x, node.y), node);
        }
    }

    Ok((nodes, (max_x, 0), empty))
}

fn find_shortest_path(
    nodes: &BTreeMap<(usize, usize), Node>,
    empty: &(usize, usize),
    target: &(usize, usize),
    ignore: &(usize, usize),
) -> usize {
    let mut seen = Vec::new();
    let mut queue = Vec::new();
    seen.push(empty.clone());
    queue.push((0, empty.clone()));

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

            let nused = match nodes.get(&ncoords) {
                Some(neighbor) => neighbor.used,
                None => continue,
            };

            let csize = match nodes.get(&coord) {
                Some(cur) => cur.size,
                None => unreachable!(),
            };

            if nused > csize {
                continue;
            }

            if ncoords.0 == target.0 && ncoords.1 == target.1 {
                return steps + 1;
            }

            seen.push(ncoords.clone());
            queue.push((steps + 1, ncoords));
        }

        i += 1;
    }

    panic!("Cannot find path from {:?} to {:?}", empty, target);
}

fn main() -> Result<(), Error> {
    let (nodes, mut goal, mut empty) = load_input(INPUT_FILE)?;

    let mut target = (goal.0 - 1, goal.1);

    let mut dist = find_shortest_path(&nodes, &empty, &target, &goal);
    dist += 1;
    goal = target.clone();
    while goal.0 != 0 || goal.1 != 0 {
        empty = (goal.0 + 1, 0);
        target = (goal.0 - 1, 0);
        dist += find_shortest_path(&nodes, &empty, &target, &goal);
        dist += 1;
        goal = target.clone();
    }

    println!("Shortest Path: {}", dist);

    Ok(())
}
