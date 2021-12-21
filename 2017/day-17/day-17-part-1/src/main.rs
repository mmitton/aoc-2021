#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::VecDeque;

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
    let mut buffer = VecDeque::new();
    let mut pos = 0;
    buffer.push_front(0);

    for iter in 1..=2017 {
        pos = (pos + steps) % buffer.len();

        buffer.insert(pos + 1, iter);
        pos += 1;
    }

    println!(
        "Steps: {}  Buffer around pos: {} {} {} ({}) {} {} {}  Pos: {}",
        steps,
        buffer[pos - 3],
        buffer[pos - 2],
        buffer[pos - 1],
        buffer[pos + 0],
        buffer[pos + 1],
        buffer[pos + 2],
        buffer[pos + 3],
        pos
    );

    println!("Answer: {}", buffer[pos + 1]);

    Ok(())
}
