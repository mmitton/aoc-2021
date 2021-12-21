#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

fn load_input(filename: &str) -> Result<usize, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let buf = BufReader::new(f);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    Ok(lines[0].parse().map_err(|e| Error::NAN(e))?)
}

fn main() -> Result<(), Error> {
    let steps = load_input(INPUT_FILE)?;
    let mut pos = 0;

    let mut answer = 0;

    for iter in 1..=50000000 {
        pos = (pos + steps) % iter;
        if pos == 0 {
            answer = iter;
        }

        pos += 1;
    }

    println!("Answer: {}", answer);

    Ok(())
}
