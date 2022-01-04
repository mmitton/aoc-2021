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

fn load_input(filename: &str) -> Result<Vec<usize>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut adapters = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        adapters.push(line.parse()?);
    }

    Ok(adapters)
}

fn main() -> Result<(), Error> {
    let mut adapters = load_input(INPUT_FILE)?;
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters[adapters.len() - 1] + 3);

    let mut differences = [0; 4];
    for i in 1..adapters.len() {
        differences[adapters[i] - adapters[i - 1]] += 1;
    }

    println!("Differences: {:?}", differences);
    println!("Answer: {}", differences[1] * differences[3]);

    Ok(())
}
