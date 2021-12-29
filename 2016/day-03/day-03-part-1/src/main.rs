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

fn load_input(filename: &str) -> Result<Vec<[usize; 3]>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut inputs = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let mut line = line.trim().to_string();
        if line == "" {
            continue;
        }

        for _ in 0..5 {
            line = line.replace("  ", " ");
        }

        let parts: Vec<&str> = line.split(" ").collect();
        inputs.push([parts[0].parse()?, parts[1].parse()?, parts[2].parse()?]);
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    let mut good = 0usize;
    for input in &inputs {
        if input[0] + input[1] > input[2]
            && input[0] + input[2] > input[1]
            && input[1] + input[2] > input[0]
        {
            good += 1;
        }
    }

    println!("Good: {}", good);

    Ok(())
}
