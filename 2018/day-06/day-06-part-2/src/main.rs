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

fn load_input(filename: &str) -> Result<Vec<(isize, isize)>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut coords = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(", ").collect();
        coords.push((parts[0].parse()?, parts[1].parse()?));
    }

    Ok(coords)
}

fn main() -> Result<(), Error> {
    let coords = load_input(INPUT_FILE)?;

    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;

    for coord in &coords {
        if coord.0 < min_x {
            min_x = coord.0;
        }
        if coord.0 > max_x {
            max_x = coord.0;
        }
        if coord.1 < min_y {
            min_y = coord.1;
        }
        if coord.1 > max_y {
            max_y = coord.1;
        }
    }

    let dx = max_x - min_x;
    let dy = max_y - min_y;
    let max_tiles = dx * dy;
    println!("{},{} => {},{}  {}", min_x, min_y, max_x, max_y, max_tiles);

    let min_x = min_x - dx;
    let max_x = max_x + dx;
    let min_y = min_y - dy;
    let max_y = max_y + dy;

    let target = if cfg!(debug_assertions) { 32 } else { 10_000 };

    let mut answer = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut dist = 0;
            for j in 0..coords.len() {
                dist += (x - coords[j].0).abs() + (y - coords[j].1).abs();
            }

            if dist < target {
                answer += 1;
            }
        }
    }

    println!("answer: {:?}", answer);

    Ok(())
}
