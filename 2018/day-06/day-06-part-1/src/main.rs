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

    let mut infinite = Vec::new();

    let mut closest = BTreeMap::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut closest_dist = isize::MAX;
            let mut closest_idx = 0;
            for j in 0..coords.len() {
                let dist = (x - coords[j].0).abs() + (y - coords[j].1).abs();
                if dist < closest_dist {
                    closest_dist = dist;
                    closest_idx = j;
                } else if dist == closest_dist {
                    closest_idx = usize::MAX;
                }
            }
            closest.insert((x, y), closest_idx);

            if x == min_x || x == max_x || y == min_y || y == max_y {
                if !infinite.contains(&closest_idx) {
                    infinite.push(closest_idx);
                }
            }
        }
    }

    let mut answer = 0;
    for i in 0..coords.len() {
        let coord = coords[i];

        if !infinite.contains(&i) {
            // Count closest
            let mut count = 0;
            for (_, idx) in &closest {
                if *idx == i {
                    count += 1;
                }
            }

            println!("{:?} has {} tiles", coord, count);
            if count < max_tiles && answer < count {
                answer = count;
            }
        }
    }

    println!("answer: {:?}", answer);

    Ok(())
}
