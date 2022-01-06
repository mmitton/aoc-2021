#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeSet;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NotADirection(String),
}

#[derive(Debug)]
enum Dir {
    NE,
    E,
    SE,
    SW,
    W,
    NW,
}

impl TryFrom<&str> for Dir {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.starts_with("ne") {
            Ok(Dir::NE)
        } else if s.starts_with("e") {
            Ok(Dir::E)
        } else if s.starts_with("se") {
            Ok(Dir::SE)
        } else if s.starts_with("sw") {
            Ok(Dir::SW)
        } else if s.starts_with("w") {
            Ok(Dir::W)
        } else if s.starts_with("nw") {
            Ok(Dir::NW)
        } else {
            Err(Error::NotADirection(s.to_string()))
        }
    }
}

impl Dir {
    fn len(&self) -> usize {
        match self {
            Dir::E | Dir::W => 1,
            _ => 2,
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<Vec<Dir>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut inputs: Vec<Vec<Dir>> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        let mut directions: Vec<Dir> = Vec::new();
        let mut line = line;
        while line.len() > 0 {
            let dir: Dir = line.try_into()?;
            line = &line[dir.len()..];
            directions.push(dir);
        }

        inputs.push(directions);
    }

    Ok(inputs)
}

fn next_day(prev: BTreeSet<(isize, isize)>) -> BTreeSet<(isize, isize)> {
    let mut next: BTreeSet<(isize, isize)> = BTreeSet::new();
    let mut queue = Vec::new();
    let directions = vec![(1, -1), (2, 0), (1, 1), (-1, 1), (-2, 0), (-1, -1)];

    for key in &prev {
        if !queue.contains(key) {
            queue.push(*key);
        }
        for dir in &directions {
            let key = (key.0 + dir.0, key.1 + dir.1);
            if !queue.contains(&key) {
                queue.push(key);
            }
        }
    }

    for tile in &queue {
        let mut adj = 0;
        for dir in &directions {
            let tile = (tile.0 + dir.0, tile.1 + dir.1);
            if prev.contains(&tile) {
                adj += 1;
            }
        }

        if prev.contains(tile) {
            if adj > 0 && adj < 3 {
                next.insert(*tile);
            }
        } else {
            if adj == 2 {
                next.insert(*tile);
            }
        }
    }

    next
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;
    let mut tiles: BTreeSet<(isize, isize)> = BTreeSet::new();

    for directions in &inputs {
        let mut x = 0isize;
        let mut y = 0isize;
        for dir in directions {
            match dir {
                Dir::NE => {
                    y -= 1;
                    x += 1;
                }
                Dir::E => x += 2,
                Dir::SE => {
                    y += 1;
                    x += 1;
                }
                Dir::SW => {
                    y += 1;
                    x -= 1;
                }
                Dir::W => x -= 2,
                Dir::NW => {
                    y -= 1;
                    x -= 1;
                }
            }
        }

        if tiles.contains(&(x, y)) {
            tiles.remove(&(x, y));
        } else {
            tiles.insert((x, y));
        }
    }

    println!("Day 0: {}", tiles.len());
    for i in 1..=100 {
        tiles = next_day(tiles);
        println!("Day {}: {}", i, tiles.len());
    }

    Ok(())
}
