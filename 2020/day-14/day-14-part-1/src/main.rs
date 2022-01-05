#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

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

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

#[derive(Debug)]
enum Op {
    Mask(usize, usize),
    Mem(usize, usize),
}

fn load_input(filename: &str) -> Result<Vec<Op>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let reader = BufReader::new(f);
    let lines = reader.lines();
    let mut ops = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let line = line.trim();

        let op = if line.starts_with("mask = ") {
            let mut or_mask = 0usize;
            let mut and_mask = 0usize;
            for c in line[7..].chars() {
                or_mask <<= 1;
                and_mask <<= 1;
                match c {
                    'X' => {
                        or_mask |= 0b0;
                        and_mask |= 0b1;
                    }
                    '1' => {
                        or_mask |= 0b1;
                        and_mask |= 0b1;
                    }
                    '0' => {
                        or_mask |= 0b0;
                        and_mask |= 0b0;
                    }
                    _ => return Err(Error::InvalidInput(line.to_string())),
                }
            }
            Op::Mask(or_mask, and_mask)
        } else if line.starts_with("mem[") {
            let line = line.replace("]", "");
            let parts = line[4..].split(" = ").collect::<Vec<&str>>();
            let addr = parts[0].parse()?;
            let num = parts[1].parse()?;

            Op::Mem(addr, num)
        } else {
            return Err(Error::InvalidInput(line.to_string()));
        };

        ops.push(op);
    }

    Ok(ops)
}

fn main() -> Result<(), Error> {
    let ops = load_input(INPUT_FILE)?;

    let mut mem = BTreeMap::new();
    let mut and_mask = !0;
    let mut or_mask = 0;

    for op in &ops {
        match op {
            Op::Mask(o, a) => {
                and_mask = *a;
                or_mask = *o
            }
            Op::Mem(addr, num) => {
                mem.insert(addr, (num | or_mask) & and_mask);
            }
        }
        println!("{:?}", op);
    }

    let mut sum = 0;
    for (addr, num) in mem.iter() {
        println!("{} = {}", addr, num);
        sum += num;
    }

    println!("Sum : {}", sum);
    Ok(())
}
