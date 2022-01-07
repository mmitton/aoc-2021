#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

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

fn load_input(filename: &str) -> Result<Vec<isize>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut changes = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        if line.starts_with("+") {
            changes.push(line[1..].parse()?);
        } else {
            changes.push(line.parse()?);
        }
    }

    Ok(changes)
}

fn main() -> Result<(), Error> {
    let changes = load_input(INPUT_FILE)?;

    let mut total = 0isize;
    for change in &changes {
        total += change;
    }

    println!("Total Change: {}", total);

    Ok(())
}
