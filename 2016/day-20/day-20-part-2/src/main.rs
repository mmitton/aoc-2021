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

fn load_input(filename: &str) -> Result<Vec<(u32, u32)>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut ranges = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.split("-").collect();
        ranges.push((parts[0].parse()?, parts[1].parse()?));
    }

    ranges.sort();
    Ok(ranges)
}

fn main() -> Result<(), Error> {
    let ranges = load_input(INPUT_FILE)?;

    let mut allows = 0;
    let mut ip = 0;
    for range in &ranges {
        println!("IP: {}  Allows: {}  Range: {:?}", ip, allows, range);
        if ip > range.1 {
            continue;
        }
        if ip < range.0 {
            allows += range.0 - ip;
        }
        if range.1 != u32::MAX {
            ip = range.1 + 1;
        } else {
            ip = u32::MAX
        }
    }
    if ip != u32::MAX {
        allows += u32::MAX - ip + 1;
    }

    println!("IP: {}  Allows: {}", ip, allows);

    Ok(())
}
