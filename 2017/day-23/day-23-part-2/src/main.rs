const INPUT_FILE: &str = "../input-2.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NotAValue(String),
    InvalidInstruction(String),
}

#[derive(Clone, Debug)]
enum Value {
    Reg(usize),
    Imm(isize),
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

#[derive(Clone, Debug)]
enum Instruction {
    Set(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Jnz(Value, Value),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "set" => Ok(Self::Set(parts[1].try_into()?, parts[2].try_into()?)),
            "sub" => Ok(Self::Sub(parts[1].try_into()?, parts[2].try_into()?)),
            "mul" => Ok(Self::Mul(parts[1].try_into()?, parts[2].try_into()?)),
            "jnz" => Ok(Self::Jnz(parts[1].try_into()?, parts[2].try_into()?)),
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

    fn c(v: &Value) -> String {
        match v {
            Value::Imm(v) => format!("{}", v),
            Value::Reg(r) => format!("{}", (*r as u8 + 'a' as u8) as char),
        }
    }

    fn imm(v: &Value) -> isize {
        match v {
            Value::Imm(v) => *v,
            Value::Reg(_) => panic!(),
        }
    }

    println!("#include <stdio.h>");
    println!("int main(int argc, char** argv) {{");
    println!("    long long a = 1;");
    println!("    long long b = 0;");
    println!("    long long c = 0;");
    println!("    long long d = 0;");
    println!("    long long e = 0;");
    println!("    long long f = 0;");
    println!("    long long g = 0;");
    println!("    long long h = 0;");

    for (pc, instruction) in instructions.iter().enumerate() {
        println!("_{}: ", pc);
        match instruction {
            Instruction::Set(x, y) => println!("    {} = {};", c(x), c(y)),
            Instruction::Sub(x, y) => println!("    {} = {} - {};", c(x), c(x), c(y)),
            Instruction::Mul(x, y) => println!("    {} = {} * {};", c(x), c(x), c(y)),
            Instruction::Jnz(x, y) => {
                println!("    if ({} != 0) goto _{};", c(x), pc as isize + imm(y))
            }
        }
        // println!("    printf(\"pc:{} a:%d b:%d c:%d d:%d e:%d f:%d g:%d h:%d\\n\", a, b, c, d, e, f, g, h);", pc);
    }

    println!("_{}:", instructions.len());
    println!("    printf(\"h: %d\\n\", h);");
    println!("}}");

    Ok(())
}
