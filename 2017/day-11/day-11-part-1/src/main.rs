#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NotADirection(String),
}

#[derive(Debug)]
enum Dir {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl TryFrom<&str> for Dir {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "n" => Ok(Dir::N),
            "ne" => Ok(Dir::NE),
            "se" => Ok(Dir::SE),
            "s" => Ok(Dir::S),
            "sw" => Ok(Dir::SW),
            "nw" => Ok(Dir::NW),
            _ => Err(Error::NotADirection(s.to_string())),
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<Vec<Dir>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut inputs: Vec<Vec<Dir>> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        let mut directions: Vec<Dir> = Vec::new();
        for dir in line.split(",") {
            directions.push(dir.try_into()?);
        }

        inputs.push(directions);
    }

    Ok(inputs)
}

fn steps_away(mut x: isize, mut y: isize) -> usize {
    let mut steps = 0;
    while x != 0 || y != 0 {
        steps += 1;
        if x == 0 {
            if y > 0 {
                y -= 2;
            } else {
                y += 2;
            }
        } else if x < 0 {
            x += 1;
            if y > 0 {
                y -= 1;
            } else {
                y += 1;
            }
        } else {
            x -= 1;
            if y > 0 {
                y -= 1;
            } else {
                y += 1;
            }
        }
        if cfg!(debug_assertions) {
            println!("Step {} -> {},{}", steps, x, y);
        }
    }

    steps
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    for directions in &inputs {
        if cfg!(debug_assertions) {
            println!("{:?}", directions);
        }

        let mut x = 0isize;
        let mut y = 0isize;
        for dir in directions {
            match dir {
                Dir::N => y -= 2,
                Dir::NE => {
                    y -= 1;
                    x += 1;
                }
                Dir::SE => {
                    y += 1;
                    x += 1;
                }
                Dir::S => y += 2,
                Dir::SW => {
                    y += 1;
                    x -= 1;
                }
                Dir::NW => {
                    y -= 1;
                    x -= 1;
                }
            }
            if cfg!(debug_assertions) {
                println!("{:?} -> {},{}", dir, x, y);
            }
        }

        println!("Steps: {}", steps_away(x, y));
    }

    Ok(())
}
