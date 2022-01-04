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

#[derive(Copy, Clone, Debug)]
struct Reg(usize);

#[derive(Copy, Clone, Debug)]
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
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
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
            "tgl" => Ok(Self::Tgl(parts[1].try_into()?)),
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
    let mut ops = load_input(INPUT_FILE)?;
    let mut registers = [0isize; 4];
    if !cfg!(debug_assertions) {
        registers[0] = 7;
    }

    let mut pc = 0isize;
    loop {
        let mut next_pc = pc + 1;

        match &ops[pc as usize] {
            Op::Cpy(x, y) => match y {
                Value::Reg(r) => registers[*r] = x.value(&registers),
                Value::Imm(_) => {}
            },
            Op::Inc(x) => match x {
                Value::Reg(r) => registers[*r] = registers[*r] + 1,
                _ => {}
            },
            Op::Dec(x) => match x {
                Value::Reg(r) => registers[*r] = registers[*r] - 1,
                _ => {}
            },
            Op::Jnz(x, y) => {
                if x.value(&registers) != 0 {
                    next_pc = pc + y.value(&registers);
                }
            }
            Op::Tgl(x) => {
                let inst = pc + x.value(&registers);
                if inst >= 0 && inst < ops.len() as isize {
                    let inst = inst as usize;

                    match &ops[inst] {
                        Op::Inc(x) => ops[inst] = Op::Dec(*x),
                        Op::Dec(x) => ops[inst] = Op::Inc(*x),
                        Op::Tgl(x) => ops[inst] = Op::Inc(*x),
                        Op::Cpy(x, y) => ops[inst] = Op::Jnz(*x, *y),
                        Op::Jnz(x, y) => ops[inst] = Op::Cpy(*x, *y),
                    }
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
