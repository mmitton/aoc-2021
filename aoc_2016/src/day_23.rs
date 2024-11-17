#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Value {
    Reg(usize),
    Imm(isize),
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
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
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
            "tgl" => Ok(Self::Tgl(parts[1].parse()?)),
            _ => Err(Error::InvalidInput(s.to_string())),
        }
    }
}

#[derive(Default)]
pub struct Day23 {
    ops: Vec<Op>,
    registers: [isize; 4],
}

impl Day23 {
    pub fn new() -> Self {
        Self::default()
    }

    fn execute(&mut self) {
        let mut pc = 0isize;
        while let Some(op) = self.ops.get(pc as usize) {
            let mut next_pc = pc + 1;

            match *op {
                Op::Cpy(x, y) => match y {
                    Value::Reg(r) => self.registers[r] = x.value(&self.registers),
                    Value::Imm(_) => {}
                },
                Op::Inc(x) => {
                    if let Value::Reg(r) = x {
                        self.registers[r] += 1;
                    }
                }
                Op::Dec(x) => {
                    if let Value::Reg(r) = x {
                        self.registers[r] -= 1;
                    }
                }
                Op::Jnz(x, y) => {
                    if x.value(&self.registers) != 0 {
                        next_pc = pc + y.value(&self.registers);
                    }
                }
                Op::Tgl(x) => {
                    let inst = pc + x.value(&self.registers);
                    if let Some(inst) = self.ops.get_mut(inst as usize) {
                        match inst {
                            Op::Inc(x) => *inst = Op::Dec(*x),
                            Op::Dec(x) => *inst = Op::Inc(*x),
                            Op::Tgl(x) => *inst = Op::Inc(*x),
                            Op::Cpy(x, y) => *inst = Op::Jnz(*x, *y),
                            Op::Jnz(x, y) => *inst = Op::Cpy(*x, *y),
                        }
                    }
                }
            }

            pc = next_pc;
        }
    }
}

impl Runner for Day23 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        for line in lines.iter() {
            self.ops.push(line.parse()?);
        }

        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day23 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.registers[0] = 7;
        self.execute();
        Ok(self.registers[0].into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.registers[0] = 12;
        self.execute();
        Ok(self.registers[0].into())
    }
}
