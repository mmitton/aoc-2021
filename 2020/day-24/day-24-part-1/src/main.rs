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

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;
    let mut tiles: BTreeSet<(isize, isize)> = BTreeSet::new();

    for directions in &inputs {
        if cfg!(debug_assertions) {
            println!("{:?}", directions);
        }

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
            if cfg!(debug_assertions) {
                println!("{:?} -> {},{}", dir, x, y);
            }
        }

        if tiles.contains(&(x, y)) {
            tiles.remove(&(x, y));
        } else {
            tiles.insert((x, y));
        }
    }

    println!("Answer: {}", tiles.len());

    Ok(())
}
