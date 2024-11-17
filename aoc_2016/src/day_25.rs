#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

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
        Err(Error::InvalidInput(s.to_string()))
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
                Err(Error::InvalidInput(s.to_string()))
            } else {
                Ok(Self::Reg((r as u8 - b'a') as usize))
            }
        } else {
            Err(Error::InvalidInput(s.to_string()))
        }
    }
}

#[derive(Clone, Debug)]
enum Op {
    Cpy(Value, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Value, Value),
    Out(Reg),
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
            "out" => Ok(Self::Out(parts[1].parse()?)),
            _ => Err(Error::InvalidInput(s.to_string())),
        }
    }
}

#[derive(Default)]
pub struct Day25 {
    ops: Vec<Op>,
}

impl Day25 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.ops.push(line.parse()?);
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day25 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        'search_loop: for initial in 0..usize::MAX {
            let mut registers = [0isize; 4];
            registers[0] = initial as isize;

            let mut state: Vec<[isize; 4]> = Vec::new();

            let mut pc = 0isize;
            loop {
                let mut next_pc = pc + 1;

                if let Some(op) = self.ops.get(pc as usize) {
                    match op {
                        Op::Cpy(x, y) => registers[y.0] = x.value(&registers),
                        Op::Inc(x) => registers[x.0] += 1,
                        Op::Dec(x) => registers[x.0] -= 1,
                        Op::Jnz(x, y) => {
                            if x.value(&registers) != 0 {
                                next_pc = pc + y.value(&registers);
                            }
                        }
                        Op::Out(x) => {
                            let output = registers[x.0];
                            if output as usize != state.len() % 2 {
                                continue 'search_loop;
                            }
                            if state.len() >= 2 && state.contains(&registers) {
                                return Ok(initial.into());
                            }
                            state.push(registers);
                        }
                    }
                } else {
                    break;
                }
                pc = next_pc;
            }
        }
        Err(Error::Unsolved)
    }
}
