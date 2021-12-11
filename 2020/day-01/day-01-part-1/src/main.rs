#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    NoSolution,
}

fn load_inputs(filename: &str) -> Result<Vec<usize>, Error> {
    let mut inputs = Vec::new();
    let file = File::open(filename).map_err(|e| Error::IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        inputs.push(line.parse::<usize>().map_err(|e| Error::NAN(e))?);
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_inputs(INPUT_FILE)?;

    for i in 0..inputs.len() {
        for j in i + 1..inputs.len() {
            if inputs[i] + inputs[j] == 2020 {
                println!("{} * {} = {}", inputs[i], inputs[j], inputs[i] * inputs[j]);
                return Ok(());
            }
        }
    }

    return Err(Error::NoSolution);
}
