#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

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

    fn value(&self, registers: &[isize; 8]) -> isize {
        match self {
            Self::Reg(r) => registers[*r],
            Self::Imm(v) => *v,
        }
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<isize>() {
            Ok(Self::Imm(num))
        } else if s.len() == 1 {
            let r = s.chars().nth(0).unwrap();
            if !r.is_ascii_lowercase() {
                Err(Error::InvalidInput(s.to_string()))
            } else {
                Ok(Self::Reg((r as u8 - b'a') as usize))
            }
        } else {
            Err(Error::InvalidInput(s.to_string()))
        }
    }
}

enum Instruction {
    Set(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Jnz(Value, Value),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "set" => Ok(Self::Set(parts[1].parse()?, parts[2].parse()?)),
            "sub" => Ok(Self::Sub(parts[1].parse()?, parts[2].parse()?)),
            "mul" => Ok(Self::Mul(parts[1].parse()?, parts[2].parse()?)),
            "jnz" => Ok(Self::Jnz(parts[1].parse()?, parts[2].parse()?)),
            _ => Err(Error::InvalidInput(s.to_string())),
        }
    }
}

#[derive(Default)]
pub struct Day23 {
    registers: [isize; 8],
    instructions: Vec<Instruction>,
    pc: isize,
}

impl Day23 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) -> (bool, usize) {
        if let Some(inst) = self.instructions.get(self.pc as usize) {
            let mut next_pc = self.pc + 1;
            let mut mul_inst = 0;
            match inst {
                Instruction::Set(x, y) => self.registers[x.as_reg()] = y.value(&self.registers),
                Instruction::Sub(x, y) => self.registers[x.as_reg()] -= y.value(&self.registers),
                Instruction::Mul(x, y) => {
                    self.registers[x.as_reg()] *= y.value(&self.registers);
                    mul_inst = 1;
                }
                Instruction::Jnz(x, y) => {
                    if x.value(&self.registers) != 0 {
                        next_pc = self.pc + y.value(&self.registers)
                    }
                }
            }
            self.pc = next_pc;
            (false, mul_inst)
        } else {
            (true, 0)
        }
    }
}

impl Runner for Day23 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.instructions.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut mul_instructions = 0;
        loop {
            let (done, mul_inst) = self.tick();
            mul_instructions += mul_inst;
            if done {
                break;
            }
        }
        Ok(mul_instructions.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut b;
        let mut c;
        let mut d;
        let mut f;
        let mut g;
        let mut h = 0;
        match (
            &self.instructions[0],
            &self.instructions[4],
            &self.instructions[5],
        ) {
            (
                Instruction::Set(Value::Reg(1), Value::Imm(set)),
                Instruction::Mul(Value::Reg(1), Value::Imm(mul)),
                Instruction::Sub(Value::Reg(1), Value::Imm(sub)),
            ) => {
                b = set * mul - sub;
            }
            _ => panic!("Cannot extract initial B value"),
        }

        c = b;
        c -= -17000;

        loop {
            f = 1;
            d = 2;
            while d < b {
                if b % d == 0 {
                    f = 0;
                    break;
                }
                d += 1;
            }
            if f == 0 {
                h += 1;
            }
            g = b - c;
            b += 17;
            if g == 0 {
                break;
            }
        }
        Ok(h.into())
    }
}
