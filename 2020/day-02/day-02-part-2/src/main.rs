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
    pos1: usize,
    pos2: usize,
    letter: char,
}

impl PasswordPolicy {
    fn is_valid(&self, s: &str) -> bool {
        fn char_match(a: char, b: char) -> u8 {
            if a == b {
                1
            } else {
                0
            }
        }

        let chars = s.chars().collect::<Vec<char>>();
        let num = char_match(chars[self.pos1 - 1], self.letter)
            + char_match(chars[self.pos2 - 1], self.letter);

        num == 1
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
        let pos1 = nums[0].parse::<usize>().map_err(|e| Error::NAN(e))?;
        let pos2 = nums[1].parse::<usize>().map_err(|e| Error::NAN(e))?;

        let password_policy = PasswordPolicy {
            pos1: pos1,
            pos2: pos2,
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
