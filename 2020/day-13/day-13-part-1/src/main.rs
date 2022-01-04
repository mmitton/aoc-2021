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

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

fn load_input(filename: &str) -> Result<(usize, Vec<usize>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let reader = BufReader::new(f);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let min_time: usize = lines[0].parse()?;
    let mut busses: Vec<usize> = Vec::new();
    for part in lines[1].split(",") {
        if part == "x" {
            continue;
        }
        busses.push(part.parse()?);
    }

    Ok((min_time, busses))
}

fn main() -> Result<(), Error> {
    let (min_time, busses) = load_input(INPUT_FILE)?;
    let mut min_wait_time = usize::MAX;
    let mut min_bus = 0;

    println!("min_time: {}  busses: {:?}", min_time, busses);
    for bus in &busses {
        let trips = (min_time + bus - 1) / bus;
        let arrive_at = trips * bus;
        let wait_time = arrive_at - min_time;
        if wait_time < min_wait_time {
            min_wait_time = wait_time;
            min_bus = *bus;
        }
    }

    println!(
        "Bus {}, Wait Time {}...  Answer: {}",
        min_bus,
        min_wait_time,
        min_bus * min_wait_time
    );
    Ok(())
}
