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

#[derive(Clone)]
struct School {
    fish: Vec<u8>,
}

impl School {
    fn after(&self, days: usize) -> usize {
        type Cache = BTreeMap<usize, usize>;
        let mut cache: Cache = BTreeMap::new();

        fn spawn(cache: &mut Cache, day: usize, days: usize) -> usize {
            // println!("spawn({}, {})", day, days);
            let mut total = 0usize;
            if day <= days {
                if let Some(day_total) = cache.get(&day) {
                    total += *day_total;
                } else {
                    let day_total = 1 + spawn(cache, day + 7, days) + spawn(cache, day + 9, days);
                    cache.insert(day, day_total);
                    total += day_total;
                }
            }

            total
        }

        let mut total = self.fish.len();
        for fish in &self.fish {
            total += spawn(&mut cache, *fish as usize + 1, days);
        }

        total
    }
}

fn load_school(filename: &str) -> Result<School, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let mut school = School { fish: Vec::new() };
    let lines = BufReader::new(f).lines();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let parts = line.split(",").collect::<Vec<&str>>();

        for part in parts {
            let state = part.parse::<u8>().map_err(|e| Error::NAN(e))?;
            school.fish.push(state);
        }
        break;
    }

    Ok(school)
}

fn main() -> Result<(), Error> {
    let school = load_school(INPUT_FILE)?;

    if cfg!(debug_assertions) {
        println!("Initial state: {:?}", school.fish);
    }

    const AFTER: usize = 256;
    println!("after({}): {}", AFTER, school.after(AFTER));
    Ok(())
}
