#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeSet;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    InvalidOp(String),
    InvalidPC(isize),
    NAN(std::num::ParseIntError),
    NoSolution,
}

#[derive(Debug, Clone)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

impl TryFrom<&str> for Op {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "nop" => Ok(Self::Nop),
            "acc" => Ok(Self::Acc),
            "jmp" => Ok(Self::Jmp),
            _ => Err(Error::InvalidOp(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Op,
    num: isize,
}

fn load_input(filename: &str) -> Result<Vec<Instruction>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let mut instructions: Vec<Instruction> = Vec::new();

    let file = File::open(filename).map_err(|e| Error::IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(Error::InvalidInput(line.to_string()));
        }

        let op: Op = parts[0].try_into()?;
        let num = parts[1].parse::<isize>().map_err(|e| Error::NAN(e))?;

        instructions.push(Instruction { op: op, num: num });
    }

    Ok(instructions)
}

fn run_program(instructions: &Vec<Instruction>) -> Result<(isize, bool), Error> {
    let mut acc = 0isize;
    let mut pc = 0isize;
    let mut seen_pc: BTreeSet<isize> = BTreeSet::new();
    loop {
        if pc < 0 || pc as usize >= instructions.len() {
            if pc as usize == instructions.len() {
                return Ok((acc, true));
            }
            return Err(Error::InvalidPC(pc));
        }

        if seen_pc.contains(&pc) {
            return Ok((acc, false));
        }
        seen_pc.insert(pc);
        match instructions[pc as usize].op {
            Op::Nop => pc += 1,
            Op::Jmp => pc += instructions[pc as usize].num,
            Op::Acc => {
                acc += instructions[pc as usize].num;
                pc += 1;
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let instructions = load_input(INPUT_FILE)?;

    for i in 0..instructions.len() {
        if let Op::Acc = instructions[i].op {
            continue;
        }

        let mut instructions = instructions.clone();
        if let Op::Nop = instructions[i].op {
            instructions[i].op = Op::Jmp;
        } else {
            instructions[i].op = Op::Nop;
        }

        let (acc, valid) = run_program(&instructions)?;
        println!("acc: {}  valid: {}", acc, valid);
        if valid {
            return Ok(());
        }
    }
    return Err(Error::NoSolution);
}
