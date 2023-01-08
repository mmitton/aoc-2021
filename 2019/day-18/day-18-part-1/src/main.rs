const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
enum Tile {
    Wall,
    Key(char),
    Door(char),
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Path {
    to: Coord,
    to_key: char,
    doors: u32,
    keys: u32,
    steps: usize,
}

fn find_paths(map: &BTreeMap<Coord, Tile>, start: Coord, paths: &mut BTreeMap<Coord, Vec<Path>>) {
    let mut local_paths: Vec<Path> = Vec::new();
    #[derive(Clone, Debug)]
    struct Job {
        at: Coord,
        path: Vec<Coord>,
        keys: u32,
        doors: u32,
    }
    let mut work: VecDeque<Job> = VecDeque::new();
    work.push_front(Job {
        at: start,
        path: Vec::new(),
        keys: 0,
        doors: 0,
    });
    let mut seen: BTreeMap<u32, Vec<Coord>> = BTreeMap::new();
    while let Some(job) = work.pop_front() {
        macro_rules! add {
            ($pos:expr, $key:expr, $door:expr) => {{
                let pos = $pos;
                let key: Option<char> = $key;
                let door: Option<char> = $door;
                let mut new_job = job.clone();
                new_job.at = pos;
                new_job.path.push(pos);
                if let Some(d) = door {
                    let d = 1 << (d as u32 - 'A' as u32);
                    new_job.doors |= d;
                }
                if let Some(k) = key {
                    // Add the path to paths
                    local_paths.push(Path {
                        to: pos,
                        to_key: k,
                        doors: new_job.doors,
                        keys: new_job.keys,
                        steps: new_job.path.len(),
                    });
                    let k = 1 << (k as u32 - 'a' as u32);
                    new_job.keys |= k;
                }
                let seen = seen.entry(new_job.doors).or_insert(Vec::new());
                if !seen.contains(&pos) {
                    seen.push(pos);
                    work.push_back(new_job);
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
                        None => add!(pos, None, None),
                        Some(Tile::Key(k)) => add!(pos, Some(*k), None),
                        Some(Tile::Door(d)) => add!(pos, None, Some(*d)),
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

fn process(start: &[Coord], map: &BTreeMap<Coord, Tile>) {
    if start.is_empty() || map.is_empty() {
        return;
    }

    print_map(start, map);

    let mut paths: BTreeMap<Coord, Vec<Path>> = BTreeMap::new();
    for start in start {
        // Find all paths from here to all keys
        find_paths(map, *start, &mut paths);
    }

    let mut key_count = 0;
    for (pos, tile) in map.iter() {
        if let Tile::Key(k) = tile {
            // Find all paths from here to other keys
            find_paths(map, *pos, &mut paths);
            if false {
                println!("Paths from {pos:?}  {k}");
                for path in paths.get(pos).unwrap().iter() {
                    println!("\t{path:?}");
                }
            }
            key_count += 1;
        }
    }

    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
    struct Job {
        steps: usize,
        keys: u32,
        pos: Vec<Coord>,
    }

    let mut work = BTreeSet::new();
    work.insert(Job {
        pos: Vec::from(start),
        keys: 0,
        steps: 0,
    });

    let mut best_steps = usize::MAX;
    while let Some(job) = work.pop_first() {
        // println!("{job:?}");
        if job.steps >= best_steps {
            continue;
        }
        for (idx, pos) in job.pos.iter().enumerate() {
            // Look for all open paths from pos
            for path in paths.get(pos).unwrap() {
                let key_bitmask = 1 << (path.to_key as u32 - 'a' as u32);
                if job.steps + path.steps >= best_steps {
                    continue;
                }
                // Skip path if we have already been to this key
                if job.keys & key_bitmask != 0 {
                    // println!("  We have already seen this key  {path:?}");
                    continue;
                }
                // Skip path if we have not unlock all doors along this path
                if path.doors & job.keys != path.doors {
                    // println!("  We have not unlocked all doors along this path  {path:?}");
                    continue;
                }
                // Skip path if there is a key along the path that we have not already picked up
                if path.keys & job.keys != path.keys {
                    // println!("  We have not picked up all keys along this path  {path:?}");
                    continue;
                }

                let mut new_job = Job {
                    pos: job.pos.clone(),
                    keys: job.keys,
                    steps: job.steps + path.steps,
                };
                new_job.pos[idx] = path.to;
                new_job.keys |= key_bitmask;
                // println!("  {new_job:?} {}", path.to_key);
                if new_job.keys.count_ones() == key_count {
                    best_steps = new_job.steps;
                } else if new_job.steps < best_steps {
                    work.insert(new_job);
                }
            }
        }
    }

    println!("best_steps: {best_steps}");
    println!();
}

fn print_map(start: &[Coord], map: &BTreeMap<Coord, Tile>) {
    let mut min = Coord {
        x: isize::MAX,
        y: isize::MAX,
    };
    let mut max = Coord {
        x: isize::MIN,
        y: isize::MIN,
    };

    for coord in map.keys() {
        if coord.x < min.x {
            min.x = coord.x;
        }
        if coord.y < min.y {
            min.y = coord.y;
        }
        if coord.x > max.x {
            max.x = coord.x;
        }
        if coord.y > max.y {
            max.y = coord.y;
        }
    }

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let pos = Coord { x, y };
            if start.contains(&pos) {
                print!("@");
            } else {
                match map.get(&pos) {
                    Some(Tile::Wall) => print!("#"),
                    Some(Tile::Key(k)) => print!("{k}"),
                    Some(Tile::Door(d)) => print!("{d}"),
                    None => print!(" "),
                }
            }
        }
        println!();
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut y = 0;
    let mut map = BTreeMap::new();
    let mut start = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            process(&start, &map);
            start.clear();
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
                '@' => {
                    start.push(pos);
                }
                'a'..='z' => {
                    map.insert(pos, Tile::Key(c));
                }
                'A'..='Z' => {
                    map.insert(pos, Tile::Door(c));
                }
                _ => {}
            }
        }

        y += 1;
    }
    process(&start, &map);
}
