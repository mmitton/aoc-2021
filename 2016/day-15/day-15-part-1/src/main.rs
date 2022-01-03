#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidInput(String),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

#[derive(Debug)]
struct Disc {
    num: isize,
    positions: isize,
    t0: isize,
}

impl Disc {
    fn position_at(&self, t: isize) -> isize {
        let pos = (self.t0 + t + self.num) % self.positions;
        pos
    }
}

impl TryFrom<&str> for Disc {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.replace(".", "");
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 12 {
            return Err(Error::InvalidInput(s.to_string()));
        }

        Ok(Self {
            num: parts[1][1..].parse()?,
            positions: parts[3].parse()?,
            t0: parts[11].parse()?,
        })
    }
}

fn load_input(filename: &str) -> Result<Vec<Disc>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut discs: Vec<Disc> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        discs.push(line.as_str().try_into()?);
    }

    Ok(discs)
}

fn main() -> Result<(), Error> {
    let discs = load_input(INPUT_FILE)?;

    'drop_loop: for t in 0..isize::MAX {
        for disc in &discs {
            if disc.position_at(t) != 0 {
                continue 'drop_loop;
            }
        }

        println!("Win at t: {}", t);
        break;
    }

    Ok(())
}
