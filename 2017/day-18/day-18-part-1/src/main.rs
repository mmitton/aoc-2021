#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NotAValue(String),
    InvalidInstruction(String),
}

#[derive(Debug)]
enum Value {
    Reg(usize),
    Imm(isize),
}

impl Value {
    fn as_reg(&self) -> usize {
        if let Self::Reg(r) = self {
            return *r;
        }
        panic!("Not a register");
    }

    fn value(&self, registers: &Vec<isize>) -> isize {
        match self {
            Self::Reg(r) => registers[*r],
            Self::Imm(v) => *v,
        }
    }
}

impl TryFrom<&str> for Value {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(num) = s.parse::<isize>() {
            Ok(Self::Imm(num))
        } else if s.len() == 1 {
            let r = s.chars().nth(0).unwrap();
            if r < 'a' || r > 'z' {
                Err(Error::NotAValue(s.to_string()))
            } else {
                Ok(Self::Reg((r as u8 - 'a' as u8) as usize))
            }
        } else {
            Err(Error::NotAValue(s.to_string()))
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Jgz(Value, Value),
    Snd(Value),
    Rcv(Value),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "set" => Ok(Self::Set(parts[1].try_into()?, parts[2].try_into()?)),
            "add" => Ok(Self::Add(parts[1].try_into()?, parts[2].try_into()?)),
            "mul" => Ok(Self::Mul(parts[1].try_into()?, parts[2].try_into()?)),
            "mod" => Ok(Self::Mod(parts[1].try_into()?, parts[2].try_into()?)),
            "jgz" => Ok(Self::Jgz(parts[1].try_into()?, parts[2].try_into()?)),
            "snd" => Ok(Self::Snd(parts[1].try_into()?)),
            "rcv" => Ok(Self::Rcv(parts[1].try_into()?)),
            _ => Err(Error::InvalidInstruction(s.to_string())),
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<Instruction>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        instructions.push(line.try_into()?);
    }

    Ok(instructions)
}

fn main() -> Result<(), Error> {
    let instructions = load_input(INPUT_FILE)?;

    let mut registers = vec![0isize; 26];

    let mut pc = 0isize;
    let mut last_freq: Option<isize> = None;
    loop {
        let inst = &instructions[pc as usize];
        let mut next_pc = pc + 1;
        println!("{}: {:?}", pc, inst);
        match inst {
            Instruction::Set(x, y) => registers[x.as_reg()] = y.value(&registers),
            Instruction::Add(x, y) => registers[x.as_reg()] += y.value(&registers),
            Instruction::Mul(x, y) => registers[x.as_reg()] *= y.value(&registers),
            Instruction::Mod(x, y) => registers[x.as_reg()] %= y.value(&registers),
            Instruction::Jgz(x, y) => {
                if registers[x.as_reg()] > 0 {
                    next_pc = pc + y.value(&registers)
                }
            }
            Instruction::Snd(x) => last_freq = Some(x.value(&registers)),
            Instruction::Rcv(x) => {
                if x.value(&registers) != 0 {
                    println!("Last Sound Played: {:?}", last_freq);
                    break;
                }
            }
        }

        pc = next_pc;
    }

    Ok(())
}
