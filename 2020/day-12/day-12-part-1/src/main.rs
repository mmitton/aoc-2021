#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidStep(String),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

#[derive(Debug)]
enum Step {
    N(isize),
    S(isize),
    W(isize),
    E(isize),
    L(isize),
    R(isize),
    F(isize),
}

fn load_input(filename: &str) -> Result<Vec<Step>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut steps = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let num: isize = line[1..].parse()?;
        let step = match &line[0..1] {
            "N" => Step::N(num),
            "S" => Step::S(num),
            "W" => Step::W(num),
            "E" => Step::E(num),
            "L" => Step::L(num),
            "R" => Step::R(num),
            "F" => Step::F(num),
            _ => return Err(Error::InvalidStep(line.to_string())),
        };
        steps.push(step);
    }

    Ok(steps)
}

fn main() -> Result<(), Error> {
    let steps = load_input(INPUT_FILE)?;
    let mut dir = 0;
    let mut x = 0isize;
    let mut y = 0isize;

    for step in &steps {
        if cfg!(debug_assertions) {
            println!("{:?}", step);
        }

        match step {
            Step::N(d) => y -= d,
            Step::S(d) => y += d,
            Step::W(d) => x -= d,
            Step::E(d) => x += d,
            Step::L(d) => dir -= d,
            Step::R(d) => dir += d,
            Step::F(d) => match dir {
                0 => x += d,
                90 => y += d,
                180 => x -= d,
                270 => y -= d,
                _ => panic!(),
            },
        }

        dir = dir % 360;
        if dir < 0 {
            dir += 360;
        }
    }

    println!("{},{} => {}", x, y, x.abs() + y.abs());

    Ok(())
}
