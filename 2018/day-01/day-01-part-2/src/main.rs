#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample-2.txt";
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

fn load_input(filename: &str) -> Result<Vec<isize>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut changes = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let parts = line.split(", ").collect::<Vec<&str>>();
        for part in &parts {
            if part.starts_with("+") {
                changes.push(part[1..].parse()?);
            } else {
                changes.push(part.parse()?);
            }
        }

        if parts.len() > 1 {
            break;
        }
    }

    Ok(changes)
}

fn main() -> Result<(), Error> {
    let changes = load_input(INPUT_FILE)?;
    let mut seen = vec![0isize];

    println!("changes: {:?}", changes);

    let mut total = 0isize;
    loop {
        for change in &changes {
            total += change;
            if seen.contains(&total) {
                println!("Seen twice: {}", total);
                return Ok(());
            }
            seen.push(total);
        }
    }
}
