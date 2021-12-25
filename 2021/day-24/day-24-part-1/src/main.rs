#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-stacey.txt";
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
            if r < 'w' || r > 'z' {
                Err(Error::NotAValue(s.to_string()))
            } else {
                Ok(Self::Reg((r as u8 - 'w' as u8) as usize))
            }
        } else {
            Err(Error::NotAValue(s.to_string()))
        }
    }
}

#[derive(Clone, Debug)]
enum Op {
    Inp(Reg),
    Add(Reg, Value),
    Mul(Reg, Value),
    Div(Reg, Value),
    Mod(Reg, Value),
    Eql(Reg, Value),
}

impl TryFrom<&str> for Op {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "inp" => Ok(Self::Inp(parts[1].try_into()?)),
            "add" => Ok(Self::Add(parts[1].try_into()?, parts[2].try_into()?)),
            "mul" => Ok(Self::Mul(parts[1].try_into()?, parts[2].try_into()?)),
            "div" => Ok(Self::Div(parts[1].try_into()?, parts[2].try_into()?)),
            "mod" => Ok(Self::Mod(parts[1].try_into()?, parts[2].try_into()?)),
            "eql" => Ok(Self::Eql(parts[1].try_into()?, parts[2].try_into()?)),
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

    let mut stack = std::collections::VecDeque::new();

    fn add_const(op: &Op) -> isize {
        match op {
            Op::Add(_, b) => b.imm(),
            _ => panic!("NOT RIGHT"),
        }
    }

    let mut values = vec![(0, 0); 14];

    for i in 0..14 {
        let idx = i * 18;
        let push_pop = &ops[idx + 5];
        let constant = &ops[idx + 15];

        let push_pop = add_const(&push_pop);
        let constant = add_const(&constant);

        if push_pop > 0 {
            // PUSH
            stack.push_back((i, constant));
        } else {
            // POP
            let last = stack.pop_back().unwrap();

            values[i] = (last.0, last.1 + push_pop);
            values[last.0] = (i, -(last.1 + push_pop));
        }
    }

    let mut largest = vec![None; 14];

    for i in 0..14 {
        if let Some(v) = largest[values[i].0] {
            largest[i] = Some(v + values[i].1);
        } else {
            if values[i].1 >= 0 {
                largest[i] = Some(9);
            } else {
                largest[i] = Some(9 + values[i].1);
            }
        }

        println!("n[{}] = n[{}] + {}", i, values[i].0, values[i].1);
    }

    print!("Largest: ");
    for i in 0..14 {
        print!("{}", largest[i].unwrap());
    }
    println!();

    Ok(())
}
