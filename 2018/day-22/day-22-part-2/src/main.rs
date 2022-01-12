#![feature(map_first_last)]

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidInput(String),
    NoSolution,
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

fn load_input(filename: &str) -> Result<(usize, (usize, usize)), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut depth = 0;
    let mut target = (0, 0);

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        if line.starts_with("depth: ") {
            depth = line[7..].parse()?;
        } else if line.starts_with("target: ") {
            let parts: Vec<&str> = line[8..].split(",").collect();
            target.0 = parts[0].parse()?;
            target.1 = parts[1].parse()?;
        } else {
            return Err(Error::InvalidInput(line.to_string()));
        }
    }

    Ok((depth, target))
}

fn erosion_level(
    geologic_index: &mut Vec<Vec<usize>>,
    coord: (usize, usize),
    target: (usize, usize),
    depth: usize,
) -> usize {
    while geologic_index.len() <= coord.1 {
        geologic_index.push(Vec::new());
    }
    while geologic_index[coord.1].len() <= coord.0 {
        geologic_index[coord.1].push(usize::MAX);
    }

    if geologic_index[coord.1][coord.0] == usize::MAX {
        if coord == (0, 0) || coord == target {
            geologic_index[coord.1][coord.0] = 0;
        } else if coord.1 == 0 {
            geologic_index[coord.1][coord.0] = coord.0 * 16807;
        } else if coord.0 == 0 {
            geologic_index[coord.1][coord.0] = coord.1 * 48271;
        } else {
            let a = erosion_level(geologic_index, (coord.0 - 1, coord.1), target, depth);
            let b = erosion_level(geologic_index, (coord.0, coord.1 - 1), target, depth);
            geologic_index[coord.1][coord.0] = a * b;
        }
    }
    (geologic_index[coord.1][coord.0] + depth) % 20183
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
#[repr(usize)]
enum Tool {
    None = 0,
    Torch = 1,
    Gear = 2,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
#[repr(usize)]
enum Terrain {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

fn main() -> Result<(), Error> {
    let (depth, target) = load_input(INPUT_FILE)?;
    let width = target.0 + 6;
    let height = target.1 + 6;

    let mut geologic_index = Vec::new();

    if cfg!(debug_assertions) {
        for y in 0..height {
            for x in 0..width {
                let c = if x == 0 && y == 0 {
                    'M'
                } else if x == target.0 && y == target.1 {
                    'T'
                } else {
                    let erosion_level = erosion_level(&mut geologic_index, (x, y), target, depth);
                    match erosion_level % 3 {
                        0 => '.',
                        1 => '=',
                        2 => '|',
                        _ => unreachable!(),
                    }
                };
                print!("{}", c);
            }
            println!();
        }
    }

    let mut queue: BTreeSet<(usize, Tool, (usize, usize))> = BTreeSet::new();
    let mut seen: BTreeMap<(usize, usize), usize> = BTreeMap::new();

    let mut tools = BTreeMap::new();
    tools.insert((Terrain::Rocky, Terrain::Rocky, Tool::Gear), Tool::Gear);
    tools.insert((Terrain::Rocky, Terrain::Rocky, Tool::Torch), Tool::Torch);
    tools.insert((Terrain::Rocky, Terrain::Wet, Tool::Gear), Tool::Gear);
    tools.insert((Terrain::Rocky, Terrain::Wet, Tool::Torch), Tool::Gear);
    tools.insert((Terrain::Rocky, Terrain::Narrow, Tool::Gear), Tool::Torch);
    tools.insert((Terrain::Rocky, Terrain::Narrow, Tool::Torch), Tool::Torch);

    tools.insert((Terrain::Wet, Terrain::Wet, Tool::Gear), Tool::Gear);
    tools.insert((Terrain::Wet, Terrain::Wet, Tool::None), Tool::None);
    tools.insert((Terrain::Wet, Terrain::Rocky, Tool::Gear), Tool::Gear);
    tools.insert((Terrain::Wet, Terrain::Rocky, Tool::None), Tool::Gear);
    tools.insert((Terrain::Wet, Terrain::Narrow, Tool::Gear), Tool::None);
    tools.insert((Terrain::Wet, Terrain::Narrow, Tool::None), Tool::None);

    tools.insert((Terrain::Narrow, Terrain::Narrow, Tool::None), Tool::None);
    tools.insert((Terrain::Narrow, Terrain::Narrow, Tool::Torch), Tool::Torch);
    tools.insert((Terrain::Narrow, Terrain::Rocky, Tool::None), Tool::Torch);
    tools.insert((Terrain::Narrow, Terrain::Rocky, Tool::Torch), Tool::Torch);
    tools.insert((Terrain::Narrow, Terrain::Wet, Tool::None), Tool::None);
    tools.insert((Terrain::Narrow, Terrain::Wet, Tool::Torch), Tool::None);

    assert!(tools.len() == 6 * 3);

    queue.insert((0, Tool::Torch, (0, 0)));
    seen.insert((0, 0), 0);

    while queue.len() > 0 {
        let (time, tool, coord) = queue.pop_first().unwrap();
        // println!("{} {:?} {:?}", time, tool, coord);

        if coord == target {
            if tool != Tool::Torch {
                queue.insert((time + 7, tool, coord));
                continue;
            }
            println!("Can get to the target in {} minutes", time);
            return Ok(());
        }

        if let Some(seen_time) = seen.get(&coord) {
            if *seen_time + 10 < time {
                continue;
            }
        } else {
            seen.insert(coord, time);
        }

        let cur_terrain: Terrain =
            match erosion_level(&mut geologic_index, coord, target, depth) % 3 {
                0 => Terrain::Rocky,
                1 => Terrain::Wet,
                2 => Terrain::Narrow,
                _ => unreachable!(),
            };

        for dir in 0..4 {
            let new_coord = match dir {
                0 => {
                    if coord.1 == 0 {
                        continue;
                    }
                    (coord.0, coord.1 - 1)
                }
                1 => {
                    if coord.0 == 0 {
                        continue;
                    }
                    (coord.0 - 1, coord.1)
                }
                2 => (coord.0, coord.1 + 1),
                3 => (coord.0 + 1, coord.1),
                _ => unreachable!(),
            };

            let new_terrain: Terrain =
                match erosion_level(&mut geologic_index, new_coord, target, depth) % 3 {
                    0 => Terrain::Rocky,
                    1 => Terrain::Wet,
                    2 => Terrain::Narrow,
                    _ => unreachable!(),
                };

            let new_tool = tools.get(&(cur_terrain, new_terrain, tool)).unwrap();
            let new_tool = *new_tool;
            let new_time = if new_tool == tool { time + 1 } else { time + 8 };
            // println!(
            //     "{:?} {:?} {:?} {} => {:?} {:?} {:?} {}",
            //     coord, cur_terrain, tool, time, new_coord, new_terrain, new_tool, new_time
            // );
            queue.insert((new_time, new_tool, new_coord));
        }
    }

    Err(Error::NoSolution)
}
