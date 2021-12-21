#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::VecDeque;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    NotAnInstruction(String),
}

#[derive(Debug)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.starts_with("s") {
            Ok(Instruction::Spin(
                s[1..].parse().map_err(|e| Error::NAN(e))?,
            ))
        } else if s.starts_with("x") {
            let parts: Vec<&str> = s[1..].split("/").collect();
            Ok(Instruction::Exchange(
                parts[0].parse().map_err(|e| Error::NAN(e))?,
                parts[1].parse().map_err(|e| Error::NAN(e))?,
            ))
        } else if s.starts_with("p") {
            let chars: Vec<char> = s.chars().collect();
            Ok(Instruction::Partner(chars[1], chars[3]))
        } else {
            Err(Error::NotAnInstruction(s.to_string()))
        }
    }
}

fn load_input(filename: &str) -> Result<(VecDeque<char>, Vec<Instruction>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let buf = BufReader::new(f);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut instructions: Vec<Instruction> = Vec::new();
    for instruction in lines[1].split(",") {
        instructions.push(instruction.try_into()?);
    }

    Ok((lines[0].chars().collect(), instructions))
}

fn print_order(programs: &VecDeque<char>, start_idx: usize) {
    for i in 0..programs.len() {
        print!("{}", programs[(start_idx + i) % programs.len()]);
    }
}

fn main() -> Result<(), Error> {
    let (mut programs, instructions) = load_input(INPUT_FILE)?;

    println!(
        "programs: {:?} {}",
        programs.iter().collect::<String>(),
        programs.len()
    );
    if cfg!(debug_assertions) {
        println!("instructions: {:?}", instructions);
    }

    for instruction in &instructions {
        match instruction {
            Instruction::Spin(size) => {
                for _ in 0..*size {
                    let p = programs.pop_back().unwrap();
                    programs.push_front(p);
                }
            }
            Instruction::Exchange(p1, p2) => {
                programs.swap(*p1, *p2);
            }
            Instruction::Partner(p1, p2) => {
                let mut idx1 = 0;
                let mut idx2 = 0;
                for i in 0..programs.len() {
                    if programs[i] == *p1 {
                        idx1 = i;
                    }
                    if programs[i] == *p2 {
                        idx2 = i;
                    }
                }
                programs.swap(idx1, idx2);
            }
        }
        print_order(&programs, 0);
        println!(" <- {:?}", instruction);
    }

    print_order(&programs, 0);
    println!();

    Ok(())
}
