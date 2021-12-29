#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidOp(String),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

#[derive(Debug)]
enum Op {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(isize),
    Jie(char, isize),
    Jio(char, isize),
}

impl TryFrom<&str> for Op {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match &s[0..3] {
            "hlf" => Ok(Self::Hlf(s.chars().nth(4).unwrap())),
            "tpl" => Ok(Self::Tpl(s.chars().nth(4).unwrap())),
            "inc" => Ok(Self::Inc(s.chars().nth(4).unwrap())),
            "jmp" => Ok(Self::Jmp(s[4..].parse()?)),
            "jie" => {
                if &s[7..8] == "+" {
                    Ok(Self::Jie(s.chars().nth(4).unwrap(), s[8..].parse()?))
                } else {
                    Ok(Self::Jie(s.chars().nth(4).unwrap(), s[7..].parse()?))
                }
            }
            "jio" => {
                if &s[7..8] == "+" {
                    Ok(Self::Jio(s.chars().nth(4).unwrap(), s[8..].parse()?))
                } else {
                    Ok(Self::Jio(s.chars().nth(4).unwrap(), s[7..].parse()?))
                }
            }
            _ => Err(Error::InvalidOp(s.to_string())),
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<Op>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut ops = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        ops.push(line.try_into()?);
    }

    Ok(ops)
}

fn main() -> Result<(), Error> {
    let ops = load_input(INPUT_FILE)?;

    let mut registers = [0usize, 0usize];
    let mut pc = 0isize;
    while pc >= 0 && pc < ops.len() as isize {
        println!("{}: {:?}", pc, ops[pc as usize]);
        let mut next_pc = pc + 1;
        match ops[pc as usize] {
            Op::Hlf(r) => registers[(r as u8 - 'a' as u8) as usize] /= 2,
            Op::Tpl(r) => registers[(r as u8 - 'a' as u8) as usize] *= 3,
            Op::Inc(r) => registers[(r as u8 - 'a' as u8) as usize] += 1,
            Op::Jmp(offset) => next_pc = pc + offset,
            Op::Jie(r, offset) => {
                if registers[(r as u8 - 'a' as u8) as usize] % 2 == 0 {
                    next_pc = pc + offset;
                }
            }
            Op::Jio(r, offset) => {
                if registers[(r as u8 - 'a' as u8) as usize] == 1 {
                    next_pc = pc + offset;
                }
            }
        }
        pc = next_pc;
    }

    println!("Register B: {}", registers[1]);
    Ok(())
}
