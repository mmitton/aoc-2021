#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    NoInput,
}

fn load_input(filename: &str) -> Result<Vec<usize>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        let mut banks = Vec::new();
        for num in line.split("\t") {
            banks.push(num.parse().map_err(|e| Error::NAN(e))?);
        }
        return Ok(banks);
    }

    Err(Error::NoInput)
}

fn realloc(banks: &Vec<usize>) -> Vec<usize> {
    let mut max_idx = 0;
    let mut banks = banks.clone();

    for i in 1..banks.len() {
        if banks[i] > banks[max_idx] {
            max_idx = i;
        }
    }

    if cfg!(debug_assertions) {
        println!("Realloc {} @ {}", banks[max_idx], max_idx);
    }
    let mut left = banks[max_idx];
    banks[max_idx] = 0;
    let mut idx = max_idx + 1;
    let num_banks = banks.len();
    while left > 0 {
        banks[idx % num_banks] += 1;
        idx += 1;
        left -= 1;
    }
    banks
}

fn main() -> Result<(), Error> {
    let banks = load_input(INPUT_FILE)?;
    println!("{:?}", banks);

    let mut steps = 0;
    let mut banks = vec![banks];
    'iter_loop: loop {
        steps += 1;
        let new_banks = realloc(&banks[banks.len() - 1]);
        println!("Step {:3} {:?}", steps, new_banks);

        for i in 0..banks.len() {
            if banks[i].eq(&new_banks) {
                println!("Loop after {} steps", steps - i);
                break 'iter_loop;
            }
        }
        banks.push(new_banks);
    }

    Ok(())
}
