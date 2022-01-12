#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidInput(String),
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

fn main() -> Result<(), Error> {
    let (depth, target) = load_input(INPUT_FILE)?;
    let width = target.0 + 6;
    let height = target.1 + 6;

    let mut geologic_index = vec![vec![usize::MAX; width]; height];

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

    let mut answer = 0;
    for y in 0..=target.1 {
        for x in 0..=target.0 {
            let erosion_level = erosion_level(&mut geologic_index, (x, y), target, depth);
            answer += erosion_level % 3;
        }
    }

    println!("Risk: {}", answer);

    Ok(())
}
