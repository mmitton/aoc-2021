#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NotAValue(String),
    NotAReg(String),
    InvalidOp(String),
}

#[derive(Clone, Debug)]
struct Reg(usize);

#[derive(Clone, Debug)]
enum Value {
    Reg(usize),
    Imm(isize),
}

impl TryFrom<&str> for Reg {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let v: Value = s.try_into()?;
        if let Value::Reg(r) = v {
            return Ok(Reg(r));
        }
        return Err(Error::NotAReg(s.to_string()));
    }
}

impl Value {
    fn imm(&self) -> isize {
        match self {
            Self::Imm(v) => *v,
            _ => panic!(),
        }
    }

    fn value(&self, registers: &[isize; 4]) -> isize {
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
            if r < 'a' || r > 'd' {
                Err(Error::NotAValue(s.to_string()))
            } else {
                Ok(Self::Reg((r as u8 - 'a' as u8) as usize))
            }
        } else {
            Err(Error::NotAValue(s.to_string()))
        }
    }
}

#[derive(Clone, Debug)]
enum Op {
    Cpy(Value, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Value, Value),
}

impl TryFrom<&str> for Op {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "cpy" => Ok(Self::Cpy(parts[1].try_into()?, parts[2].try_into()?)),
            "inc" => Ok(Self::Inc(parts[1].try_into()?)),
            "dec" => Ok(Self::Dec(parts[1].try_into()?)),
            "jnz" => Ok(Self::Jnz(parts[1].try_into()?, parts[2].try_into()?)),
            _ => Err(Error::InvalidOp(s.to_string())),
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<Op>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut ops: Vec<Op> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        ops.push(line.try_into()?);
    }

    Ok(ops)
}

fn main() -> Result<(), Error> {
    let ops = load_input(INPUT_FILE)?;
    let mut registers = [0isize; 4];
    registers[2] = 1;

    let mut pc = 0isize;
    loop {
        let mut next_pc = pc + 1;

        match &ops[pc as usize] {
            Op::Cpy(x, y) => registers[y.0] = x.value(&registers),
            Op::Inc(x) => registers[x.0] = registers[x.0] + 1,
            Op::Dec(x) => registers[x.0] = registers[x.0] - 1,
            Op::Jnz(x, y) => {
                if x.value(&registers) != 0 {
                    next_pc = pc + y.value(&registers);
                }
            }
        }

        if next_pc < 0 || next_pc >= ops.len() as isize {
            break;
        }
        pc = next_pc;
    }

    for r in 'a'..='d' {
        println!(
            "Register {}: {}",
            r,
            registers[(r as u8 - 'a' as u8) as usize]
        );
    }

    Ok(())
}
