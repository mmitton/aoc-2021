#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    ActionUnknown(String),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
enum Action {
    Forward,
    Down,
    Up,
}

impl TryFrom<&str> for Action {
    type Error = Error;

    fn try_from(s: &str) -> Result<Action, Self::Error> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(Error::ActionUnknown(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct Move {
    action: Action,
    delta: isize,
}

fn load_moves(filename: &str) -> Result<Vec<Move>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let mut moves = Vec::new();
    for line in BufReader::new(f).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidInput(line.to_string()));
        }

        let action = Action::try_from(parts[0])?;
        let delta = parts[1].parse::<isize>().map_err(|e| Error::NAN(e))?;

        moves.push(Move {
            action: action,
            delta: delta,
        });
    }

    Ok(moves)
}

fn main() {
    let moves = load_moves(INPUT_FILE).expect("Could not load input file");

    let mut horizontal = 0isize;
    let mut depth = 0isize;
    let mut aim = 0isize;
    for m in moves {
        match m.action {
            Action::Forward => {
                horizontal += m.delta;
                depth += aim * m.delta;
            }
            Action::Down => aim += m.delta,
            Action::Up => aim -= m.delta,
        }

        println!(
            "horizontal:{}  depth:{}  aim:{}  move:{:?}",
            horizontal, depth, aim, m
        );
    }

    println!("{}", horizontal * depth);
}
