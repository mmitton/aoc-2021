use std::str::FromStr;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone, Debug)]
struct Reg(usize);

#[derive(Clone, Debug)]
enum Value {
    Reg(usize),
    Imm(isize),
}

impl FromStr for Reg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Value = s.parse()?;
        if let Value::Reg(r) = v {
            return Ok(Reg(r));
        }
        Err(Error::InvalidInput(s.into()))
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

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<isize>() {
            Ok(Self::Imm(num))
        } else if s.len() == 1 {
            let r = s.chars().nth(0).unwrap();
            if !('a'..='d').contains(&r) {
                Err(Error::InvalidInput(s.into()))
            } else {
                Ok(Self::Reg((r as u8 - b'a') as usize))
            }
        } else {
            Err(Error::InvalidInput(s.into()))
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

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "cpy" => Ok(Self::Cpy(parts[1].parse()?, parts[2].parse()?)),
            "inc" => Ok(Self::Inc(parts[1].parse()?)),
            "dec" => Ok(Self::Dec(parts[1].parse()?)),
            "jnz" => Ok(Self::Jnz(parts[1].parse()?, parts[2].parse()?)),
            _ => Err(Error::InvalidInput(s.into())),
        }
    }
}

#[derive(Default)]
pub struct Day12 {
    ops: Vec<Op>,
    registers: [isize; 4],
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }

    fn run(&mut self) {
        let mut pc = 0isize;
        while let Some(op) = self.ops.get(pc as usize) {
            let mut next_pc = pc + 1;

            match op {
                Op::Cpy(x, y) => self.registers[y.0] = x.value(&self.registers),
                Op::Inc(x) => self.registers[x.0] += 1,
                Op::Dec(x) => self.registers[x.0] -= 1,
                Op::Jnz(x, y) => {
                    if x.value(&self.registers) != 0 {
                        next_pc = pc + y.value(&self.registers);
                    }
                }
            }

            pc = next_pc;
        }
    }
}

impl Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.ops.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.run();
        Ok(self.registers[0].into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.registers[2] = 1;
        self.run();
        Ok(self.registers[0].into())
    }
}
