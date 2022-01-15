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

#[derive(Debug, Copy, Clone)]
struct Coord(isize, isize, isize, isize);

impl Coord {
    fn dist_to(&self, rhs: &Self) -> isize {
        (self.0 - rhs.0).abs()
            + (self.1 - rhs.1).abs()
            + (self.2 - rhs.2).abs()
            + (self.3 - rhs.3).abs()
    }
}

fn load_input(filename: &str) -> Result<Vec<Vec<Coord>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut sets = Vec::new();
    let mut set: Vec<Coord> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            if set.len() > 0 {
                sets.push(set);
                set = Vec::new();
            }
            continue;
        }

        let parts: Vec<&str> = line.split(",").collect();
        set.push(Coord(
            parts[0].parse()?,
            parts[1].parse()?,
            parts[2].parse()?,
            parts[3].parse()?,
        ));
    }
    if set.len() > 0 {
        sets.push(set);
    }

    Ok(sets)
}

fn main() -> Result<(), Error> {
    let sets = load_input(INPUT_FILE)?;

    for set in &sets {
        let mut constellations: Vec<Vec<Coord>> = Vec::new();
        for c in set {
            constellations.push(vec![*c]);
        }

        loop {
            let mut merged = false;
            for i in 0..constellations.len() {
                let mut j = i + 1;
                loop {
                    if j >= constellations.len() {
                        break;
                    }
                    'check_loop: for ii in 0..constellations[i].len() {
                        for jj in 0..constellations[j].len() {
                            if constellations[i][ii].dist_to(&constellations[j][jj]) <= 3 {
                                // Merge i and j
                                merged = true;
                                let merge = constellations[j].clone();
                                constellations[i].extend_from_slice(&merge);
                                constellations.remove(j);
                                j -= 1;
                                break 'check_loop;
                            }
                        }
                    }
                    j += 1;
                }
            }

            if !merged {
                break;
            }
        }

        let mut answer = 0;
        for i in 0..constellations.len() {
            answer += 1;
        }

        println!("Answer: {}", answer);
    }

    Ok(())
}
