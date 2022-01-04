#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<Vec<Vec<char>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut map = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        map.push(line.chars().collect());
    }

    Ok(map)
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Path {
    dist: usize,
    final_location: usize,
    seen_locations: Vec<usize>,
}

fn find_paths(map: &Vec<Vec<char>>, sx: usize, sy: usize) -> Vec<Path> {
    let mut seen = vec![(sx, sy)];
    let mut steps = vec![(sx, sy, 0, Vec::new())];
    let mut paths = Vec::new();

    let mut i = 0;
    while i < steps.len() {
        for dir in 0..4 {
            let (x, y) = match dir {
                0 => (steps[i].0, steps[i].1 - 1),
                1 => (steps[i].0, steps[i].1 + 1),
                2 => (steps[i].0 - 1, steps[i].1),
                3 => (steps[i].0 + 1, steps[i].1),
                _ => unreachable!(),
            };

            if map[y][x] == '#' || seen.contains(&(x, y)) {
                continue;
            }

            seen.push((x, y));
            let mut step = steps[i].clone();
            step.0 = x;
            step.1 = y;
            step.2 += 1;
            if map[y][x].is_digit(10) {
                let location = (map[y][x] as u8 - '0' as u8) as usize;
                let path = Path {
                    dist: step.2,
                    final_location: location,
                    seen_locations: step.3.clone(),
                };
                paths.push(path);
                step.3.push(location);
            }
            steps.push(step);
        }

        i += 1;
    }

    paths
}

fn main() -> Result<(), Error> {
    let map = load_input(INPUT_FILE)?;
    let mut locations = vec![(0, 0); 10];
    let mut num_locations = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let c = map[y][x];
            if c.is_digit(10) {
                let num: usize = (c as u8 - '0' as u8) as usize;
                locations[num] = (x, y);
                if num >= num_locations {
                    num_locations = num + 1;
                }
            }
            if cfg!(debug_assertions) {
                print!("{}", c);
            }
        }
        if cfg!(debug_assertions) {
            println!();
        }
    }

    while locations.len() != num_locations {
        locations.remove(locations.len() - 1);
    }

    let mut paths = BTreeMap::new();
    for a in 0..num_locations {
        for path in find_paths(&map, locations[a].0, locations[a].1) {
            paths.insert((a, path.final_location), path);
        }
    }

    let mut best_run = usize::MAX;
    let mut steps = vec![(0, vec![0])];
    while steps.len() > 0 {
        steps.sort();
        let step = steps.remove(0);
        let cur_location = step.1[step.1.len() - 1];
        println!(
            "At {} after {} steps, have seen {:?}",
            cur_location, step.0, step.1
        );

        if step.1.len() == num_locations {
            let path = paths.get(&(cur_location, 0)).unwrap();
            let dist = step.0 + path.dist;
            println!("Found path with {} steps", dist);
            if dist < best_run {
                best_run = dist;
            }
            continue;
        }

        for loc in 0..num_locations {
            if step.1.contains(&loc) {
                continue;
            }

            let mut new_step = step.clone();
            let path = paths.get(&(cur_location, loc)).unwrap();
            for seen in &path.seen_locations {
                if !new_step.1.contains(seen) {
                    new_step.1.push(*seen);
                }
            }
            new_step.1.push(loc);
            new_step.0 += path.dist;

            steps.push(new_step);
        }
    }

    println!("Best Path: {}", best_run);

    Ok(())
}
