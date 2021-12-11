#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
}

#[derive(Debug)]
struct Map {
    lines: Vec<String>,
}

impl Map {
    fn get_xy(&self, x: usize, y: usize) -> char {
        let line = &self.lines[y];
        let chars = line.chars().collect::<Vec<char>>();
        chars[x % chars.len()]
    }
}

fn load_map(filename: &str) -> Result<Map, Error> {
    let mut map = Map { lines: Vec::new() };
    let file = File::open(filename).map_err(|e| Error::IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if map.lines.len() != 0 && map.lines[0].len() != line.len() {
            return Err(Error::InvalidInput(line.to_string()));
        }

        map.lines.push(line.to_string());
    }

    Ok(map)
}

fn main() -> Result<(), Error> {
    let map = load_map(INPUT_FILE)?;

    let mut num_trees = 0usize;
    for y in 1..map.lines.len() {
        match map.get_xy(3 * y, y) {
            '#' => num_trees += 1,
            _ => {}
        }
    }

    println!("num trees: {}", num_trees);
    return Ok(());
}
