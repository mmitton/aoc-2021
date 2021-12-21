#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

fn load_input(filename: &str) -> Result<(usize, usize), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let mut lines = BufReader::new(f).lines().map(|l| l.unwrap());

    let a = lines.next().unwrap()[24..]
        .parse()
        .map_err(|e| Error::NAN(e))?;
    let b = lines.next().unwrap()[24..]
        .parse()
        .map_err(|e| Error::NAN(e))?;

    Ok((a, b))
}

fn main() -> Result<(), Error> {
    let (mut a, mut b) = load_input(INPUT_FILE)?;

    const ROUNDS: usize = 40_000_000;
    let mut matches = 0;
    for _ in 0..ROUNDS {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        if a & 0xFFFF == b & 0xFFFF {
            matches += 1;
        }
    }
    println!("Matches after {} round: {}", ROUNDS, matches);

    Ok(())
}
