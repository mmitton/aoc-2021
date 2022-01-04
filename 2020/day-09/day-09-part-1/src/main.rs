#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    NoSolution,
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

fn load_input(filename: &str) -> Result<(usize, Vec<usize>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut numbers = Vec::new();
    let mut preamble = None;

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        let number = line.parse()?;
        if preamble.is_none() {
            preamble = Some(number);
        } else {
            numbers.push(number);
        }
    }

    Ok((preamble.unwrap(), numbers))
}

fn main() -> Result<(), Error> {
    let (preamble, numbers) = load_input(INPUT_FILE)?;

    'search_loop: for i in preamble..numbers.len() {
        if cfg!(debug_assertions) {
            println!("Considering {}", numbers[i]);
        }
        for j in i - preamble..i {
            for k in j + 1..i {
                if numbers[j] + numbers[k] == numbers[i] {
                    continue 'search_loop;
                }
            }
        }

        println!("{} is invalid", numbers[i]);
        return Ok(());
    }

    Err(Error::NoSolution)
}
