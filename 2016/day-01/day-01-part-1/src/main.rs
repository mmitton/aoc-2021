#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidStep(String),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

struct Step {
    turn: char,
    dist: isize,
}

impl TryFrom<&str> for Step {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match &s[0..1] {
            "R" | "L" => Ok(Step {
                turn: s.chars().nth(0).unwrap(),
                dist: s[1..].parse()?,
            }),
            _ => Err(Error::InvalidStep(s.to_string())),
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<Vec<Step>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut inputs = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut steps: Vec<Step> = Vec::new();
        for step in line.split(", ") {
            steps.push(step.try_into()?);
        }

        inputs.push(steps);
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    for steps in &inputs {
        let mut dir = 0isize;
        let mut x = 0isize;
        let mut y = 0isize;

        for step in steps {
            match step.turn {
                'L' => dir -= 1,
                'R' => dir += 1,
                _ => unreachable!(),
            }
            if dir < 0 {
                dir = 3;
            } else if dir > 3 {
                dir = 0;
            }

            match dir {
                0 => y -= step.dist,
                1 => x += step.dist,
                2 => y += step.dist,
                3 => x -= step.dist,
                _ => unreachable!(),
            }
        }

        println!("x:{}  y:{}  Answer:{}", x, y, x.abs() + y.abs());
    }

    Ok(())
}
