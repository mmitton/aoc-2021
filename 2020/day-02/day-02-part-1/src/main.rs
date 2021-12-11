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
    InvalidInput(String),
}

#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

impl PasswordPolicy {
    fn is_valid(&self, s: &str) -> bool {
        let num_letters = s
            .chars()
            .filter(|c| *c == self.letter)
            .collect::<Vec<char>>()
            .len();
        num_letters >= self.min && num_letters <= self.max
    }
}

fn load_inputs(filename: &str) -> Result<Vec<(PasswordPolicy, String)>, Error> {
    let mut inputs = Vec::new();
    let file = File::open(filename).map_err(|e| Error::IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 3 {
            return Err(Error::InvalidInput(line.to_string()));
        }
        let nums: Vec<&str> = parts[0].split('-').collect();
        if nums.len() != 2 {
            return Err(Error::InvalidInput(line.to_string()));
        }
        let min = nums[0].parse::<usize>().map_err(|e| Error::NAN(e))?;
        let max = nums[1].parse::<usize>().map_err(|e| Error::NAN(e))?;

        let password_policy = PasswordPolicy {
            min: min,
            max: max,
            letter: parts[1].chars().nth(0).unwrap(),
        };

        inputs.push((password_policy, parts[2].to_string()));
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_inputs(INPUT_FILE)?;

    let mut valid = 0usize;
    for input in &inputs {
        if input.0.is_valid(&input.1) {
            valid += 1;
        }
    }

    println!("num valid: {}", valid);
    return Ok(());
}
