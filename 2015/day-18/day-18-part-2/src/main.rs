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
    fn from(e: std::num::ParseIntError) -> Self {
        Self::NAN(e)
    }
}

struct Lights {
    num_on: usize,
    light_on: Vec<Vec<bool>>,
    neighbors_on: Vec<Vec<u8>>,
}

impl Lights {
    fn next(&mut self) {
        let max_r = self.light_on.len() - 1;
        let max_c = self.light_on[0].len() - 1;
        for r in 0..=max_r {
            for c in 0..=max_c {
                self.neighbors_on[r][c] = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr < 0 || nc < 0 || nr > max_r as isize || nc > max_c as isize {
                            continue;
                        }

                        self.neighbors_on[r][c] += if self.light_on[nr as usize][nc as usize] {
                            1
                        } else {
                            0
                        };
                    }
                }
            }
        }
        self.num_on = 0;
        for r in 0..=max_r {
            for c in 0..=max_c {
                if (r == 0 && (c == 0 || c == max_c)) || (r == max_r && (c == 0 || c == max_c)) {
                    self.num_on += 1;
                    self.light_on[r][c] = true;
                } else if self.light_on[r][c] {
                    // stays on with 2 or 3 of neighbors on
                    if self.neighbors_on[r][c] == 2 || self.neighbors_on[r][c] == 3 {
                        self.num_on += 1;
                        self.light_on[r][c] = true;
                    } else {
                        self.light_on[r][c] = false;
                    }
                } else {
                    // Turns on with 3 neighbors on
                    if self.neighbors_on[r][c] == 3 {
                        self.num_on += 1;
                        self.light_on[r][c] = true;
                    } else {
                        self.light_on[r][c] = false;
                    }
                }
            }
        }
    }
}

impl std::fmt::Debug for Lights {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for r in 0..self.light_on.len() {
            for c in 0..self.light_on[r].len() {
                write!(fmt, "{}", if self.light_on[r][c] { "#" } else { "." })?;
            }
            if cfg!(debug_assertions) {
                write!(fmt, "   ")?;
                for c in 0..self.neighbors_on[r].len() {
                    write!(fmt, "{}", self.neighbors_on[r][c])?;
                }
            }
            writeln!(fmt)?;
        }

        Ok(())
    }
}

fn load_input(filename: &str) -> Result<Lights, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut lights = Lights {
        num_on: 0,
        light_on: Vec::new(),
        neighbors_on: Vec::new(),
    };

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut light_on = Vec::new();
        let mut neighbors_on = Vec::new();
        for c in line.chars() {
            neighbors_on.push(0);
            if c == '.' {
                light_on.push(false);
            } else if c == '#' {
                light_on.push(true);
            }
        }

        lights.light_on.push(light_on);
        lights.neighbors_on.push(neighbors_on);
    }

    let max_r = lights.light_on.len() - 1;
    let max_c = lights.light_on[0].len() - 1;
    lights.light_on[0][0] = true;
    lights.light_on[0][max_c] = true;
    lights.light_on[max_r][0] = true;
    lights.light_on[max_r][max_c] = true;

    Ok(lights)
}

fn main() -> Result<(), Error> {
    let mut lights = load_input(INPUT_FILE)?;

    const ITERS: usize = if cfg!(debug_assertions) { 5 } else { 100 };

    for i in 0..ITERS {
        if i == 0 {
            println!("Initial:\n{:?}", lights);
        } else {
            println!("Step {}:\n{:?}", i, lights);
        }
        lights.next();
    }

    println!("Final:\n{:?}", lights);
    println!(
        "Number of lights on after {} steps: {}",
        ITERS, lights.num_on
    );

    Ok(())
}
