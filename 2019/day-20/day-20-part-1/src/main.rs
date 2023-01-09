const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Char(char),
    Path,
    Warp(String, bool, Option<Coord>),
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Path {
    steps: usize,
    warp_to: Option<Coord>,
}

fn find_paths(map: &BTreeMap<Coord, Tile>, start: Coord, paths: &mut BTreeMap<Coord, Vec<Path>>) {
    let mut local_paths: Vec<Path> = Vec::new();
    #[derive(Clone, Debug)]
    struct Job {
        at: Coord,
        path: Vec<Coord>,
    }
    let mut work: VecDeque<Job> = VecDeque::new();
    work.push_front(Job {
        at: start,
        path: Vec::new(),
    });
    let mut seen: BTreeSet<Coord> = BTreeSet::new();
    while let Some(job) = work.pop_front() {
        macro_rules! add {
            ($pos:expr, $warp:expr) => {{
                let pos = $pos;
                if !seen.contains(&pos) {
                    seen.insert(pos);
                    let warp: Option<(&String, &Option<Coord>)> = $warp;
                    if let Some(warp) = warp {
                        if warp.0 != "AA" {
                            local_paths.push(Path {
                                steps: job.path.len() + 1 + if warp.1.is_some() { 1 } else { 0 },
                                warp_to: *warp.1,
                            })
                        }
                    } else {
                        let mut new_job = job.clone();
                        new_job.at = pos;
                        new_job.path.push(pos);
                        work.push_back(new_job);
                    }
                }
            }};
        }
        macro_rules! check {
            ($delta:expr) => {{
                let pos = Coord {
                    x: job.at.x + $delta.0,
                    y: job.at.y + $delta.1,
                };
                if pos != start && !job.path.contains(&pos) {
                    match map.get(&pos) {
                        Some(Tile::Wall) => {}
                        Some(Tile::Path) => add!(pos, None),
                        Some(Tile::Warp(from, _inside_warp, to)) => add!(pos, Some((from, to))),
                        Some(Tile::Char(_)) => {}
                        None => unreachable!(),
                    }
                }
            }};
        }

        check!((-1, 0));
        check!((1, 0));
        check!((0, -1));
        check!((0, 1));
    }

    local_paths.sort_by_key(|path| path.steps);
    local_paths.reverse();
    paths.insert(start, local_paths);
}

fn print_map(map: &BTreeMap<Coord, Tile>) {
    let mut min = Coord {
        x: isize::MAX,
        y: isize::MAX,
    };
    let mut max = Coord {
        x: isize::MIN,
        y: isize::MIN,
    };

    for c in map.keys() {
        if c.x < min.x {
            min.x = c.x;
        }
        if c.x > max.x {
            max.x = c.x;
        }
        if c.y < min.y {
            min.y = c.y;
        }
        if c.y > max.y {
            max.y = c.y;
        }
    }

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            match map.get(&Coord { x, y }) {
                Some(Tile::Wall) => print!("#"),
                Some(Tile::Path) | None => print!(" "),
                Some(Tile::Char(c)) => print!("{c}"),
                Some(Tile::Warp(_, inside, _)) => print!("{}", if *inside { '+' } else { '-' }),
            }
        }
        println!();
    }
}

fn process(map: &mut BTreeMap<Coord, Tile>) {
    if map.is_empty() {
        return;
    }

    println!();

    let mut min = Coord {
        x: isize::MAX,
        y: isize::MAX,
    };
    let mut max = Coord {
        x: isize::MIN,
        y: isize::MIN,
    };

    for (c, t) in map.iter() {
        if let Tile::Path = t {
            if c.x < min.x {
                min.x = c.x;
            }
            if c.x > max.x {
                max.x = c.x;
            }
            if c.y < min.y {
                min.y = c.y;
            }
            if c.y > max.y {
                max.y = c.y;
            }
        }
    }

    let mut warps: BTreeMap<String, Vec<(Coord, bool)>> = BTreeMap::new();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if let Some(Tile::Path) = map.get(&Coord { x, y }) {
                macro_rules! check {
                    ($x1:expr, $y1:expr, $x2:expr, $y2:expr) => {
                        match (
                            map.get(&Coord {
                                x: x + $x1,
                                y: y + $y1,
                            }),
                            map.get(&Coord {
                                x: x + $x2,
                                y: y + $y2,
                            }),
                        ) {
                            (Some(Tile::Char(c1)), Some(Tile::Char(c2))) => {
                                let outside_warp =
                                    x == min.x || x == max.x || y == min.y || y == max.y;

                                warps
                                    .entry(format!("{c1}{c2}"))
                                    .or_insert(Vec::new())
                                    .push((Coord { x, y }, !outside_warp));
                            }
                            _ => {}
                        }
                    };
                }
                // Check for warps
                check!(-2, 0, -1, 0);
                check!(1, 0, 2, 0);
                check!(0, -2, 0, -1);
                check!(0, 1, 0, 2);
            }
        }
    }
    for (label, coords) in warps.iter() {
        if label == "AA" || label == "ZZ" {
            assert!(coords.len() == 1);
            map.insert(coords[0].0, Tile::Warp(label.clone(), coords[0].1, None));
        } else {
            assert!(coords.len() == 2);
            map.insert(
                coords[0].0,
                Tile::Warp(label.clone(), coords[0].1, Some(coords[1].0)),
            );
            map.insert(
                coords[1].0,
                Tile::Warp(label.clone(), coords[1].1, Some(coords[0].0)),
            );
        }
    }

    print_map(map);

    let mut paths: BTreeMap<Coord, Vec<Path>> = BTreeMap::new();
    for coords in warps.values() {
        for coord in coords.iter() {
            find_paths(map, coord.0, &mut paths);
        }
    }

    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
    struct Job {
        steps: usize,
        pos: Option<Coord>,
    }

    let mut work = BTreeSet::new();
    work.insert(Job {
        pos: Some(warps.get("AA").unwrap()[0].0),
        steps: 0,
    });

    while let Some(job) = work.pop_first() {
        // println!("{job:?}");
        if job.pos.is_none() {
            println!("ans: {}", job.steps);
            return;
        }
        let pos = job.pos.unwrap();
        for path in paths.get(&pos).unwrap() {
            let new_job = Job {
                pos: path.warp_to,
                steps: job.steps + path.steps,
            };
            work.insert(new_job);
        }
    }

    unreachable!();
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut y = 0;
    let mut map = BTreeMap::new();
    for line in lines.iter() {
        if line.is_empty() {
            process(&mut map);
            map.clear();
            y = 0;
            continue;
        }

        for (x, c) in line.chars().enumerate() {
            let pos = Coord { x: x as isize, y };
            match c {
                '#' => {
                    map.insert(pos, Tile::Wall);
                }
                '.' => {
                    map.insert(pos, Tile::Path);
                }
                'A'..='Z' => {
                    map.insert(pos, Tile::Char(c));
                }
                _ => {}
            }
        }

        y += 1;
    }
    process(&mut map);
}
