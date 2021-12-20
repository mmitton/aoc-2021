#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

fn load_input(filename: &str) -> Result<Vec<isize>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut jumps: Vec<isize> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        jumps.push(line.parse().map_err(|e| Error::NAN(e))?);
    }

    Ok(jumps)
}

fn main() -> Result<(), Error> {
    let mut jumps = load_input(INPUT_FILE)?;

    let mut idx = 0isize;
    let mut steps = 0usize;
    loop {
        if cfg!(debug_assertions) {
            println!("idx: {}  jumps: {:?}", idx, jumps);
        }
        let new_idx = jumps[idx as usize] + idx;
        jumps[idx as usize] += 1;

        idx = new_idx;
        steps += 1;

        if idx < 0 || idx >= jumps.len() as isize {
            break;
        }
    }

    println!("Steps: {}", steps);

    Ok(())
}
