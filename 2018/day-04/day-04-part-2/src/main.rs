#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

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

fn load_input(filename: &str) -> Result<BTreeMap<usize, Vec<(usize, usize)>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let buf = BufReader::new(f);
    let mut lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    lines.sort();

    let mut sleep_times = BTreeMap::new();
    let mut guard: usize = 0;
    let mut sleep_time: usize = 0;

    for line in &lines {
        if line.contains("begins shift") {
            let parts: Vec<&str> = line.split(" ").collect();
            guard = parts[3][1..].parse()?;
        } else if line.contains("falls") {
            sleep_time = line[15..17].parse()?;
        } else if line.contains("wakes") {
            let wake_time: usize = line[15..17].parse()?;

            if !sleep_times.contains_key(&guard) {
                sleep_times.insert(guard, Vec::new());
            }

            sleep_times
                .get_mut(&guard)
                .unwrap()
                .push((sleep_time, wake_time));
        }
    }

    Ok(sleep_times)
}

fn main() -> Result<(), Error> {
    let sleep_times = load_input(INPUT_FILE)?;

    let mut max_cnt = 0;
    let mut max_guard = 0;
    let mut max_min = 0;

    for (guard, sleep_times) in &sleep_times {
        let mut minutes = vec![0; 60];

        for sleep_time in sleep_times {
            for min in sleep_time.0..sleep_time.1 {
                minutes[min] += 1;
                if minutes[min] > max_cnt {
                    max_cnt = minutes[min];
                    max_min = min;
                    max_guard = *guard;
                }
            }
        }
    }

    println!("{} * {} = {}", max_guard, max_min, max_min * max_guard);
    Ok(())
}
