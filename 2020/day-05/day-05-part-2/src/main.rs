#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    NoSolution,
}

#[allow(dead_code)]
#[derive(Debug)]
struct BoardingPass {
    pass: String,
    row: usize,
    column: usize,
    seat_id: usize,
}

impl BoardingPass {
    fn new(pass: &str) -> Result<Self, Error> {
        fn find_position(pass: &str) -> Result<usize, Error> {
            let mut pos = 0usize;
            let mut range = 1 << pass.len();
            for c in pass.chars() {
                range /= 2;
                match c {
                    'F' | 'L' => pos += 0,
                    'B' | 'R' => pos += range,
                    _ => return Err(Error::InvalidInput(pass.to_string())),
                }
            }
            Ok(pos)
        }

        let row = find_position(&pass[0..7])?;
        let column = find_position(&pass[7..])?;

        Ok(Self {
            pass: pass.to_string(),
            row: row,
            column: column,
            seat_id: (row * 8) + column,
        })
    }
}

fn load_input(filename: &str) -> Result<Vec<BoardingPass>, Error> {
    let mut passes: Vec<BoardingPass> = Vec::new();

    let file = File::open(filename).map_err(|e| Error::IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        passes.push(BoardingPass::new(line)?);
    }

    Ok(passes)
}

fn main() -> Result<(), Error> {
    let mut passes = load_input(INPUT_FILE)?;

    passes.sort_by_key(|p| p.seat_id);
    for i in 0..passes.len() - 1 {
        let (row, column) = if passes[i].column == 7 {
            (passes[i].row + 1, 0)
        } else {
            (passes[i].row, passes[i].column + 1)
        };

        if passes[i + 1].row != row || passes[i + 1].column != column {
            println!(
                "My seat is row:{}  col:{}  seat_id:{}",
                row,
                column,
                (row * 8) + column
            );
            return Ok(());
        }
    }

    return Err(Error::NoSolution);
}
