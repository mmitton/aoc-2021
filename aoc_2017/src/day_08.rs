#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

enum Op {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Ne,
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Op::Gt),
            ">=" => Ok(Op::Gte),
            "<" => Ok(Op::Lt),
            "<=" => Ok(Op::Lte),
            "==" => Ok(Op::Eq),
            "!=" => Ok(Op::Ne),
            _ => Err(Error::InvalidInput(format!("Unknown Op {s:?}"))),
        }
    }
}

struct Instruction {
    dest_reg: usize,
    inc: bool,
    delta: isize,
    comp_reg: usize,
    comp_op: Op,
    comp_imm: isize,
}

#[derive(Default)]
pub struct Day08 {
    instructions: Vec<Instruction>,
    registers: Vec<isize>,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }

    fn run(&mut self) -> (isize, isize) {
        let mut max = isize::MIN;
        for inst in self.instructions.iter() {
            let mut dest_reg = self.registers[inst.dest_reg];
            let comp_reg = self.registers[inst.comp_reg];
            let delta = if !inst.inc { -inst.delta } else { inst.delta };

            match inst.comp_op {
                Op::Gt => {
                    if comp_reg > inst.comp_imm {
                        dest_reg += delta
                    }
                }
                Op::Gte => {
                    if comp_reg >= inst.comp_imm {
                        dest_reg += delta
                    }
                }
                Op::Lt => {
                    if comp_reg < inst.comp_imm {
                        dest_reg += delta
                    }
                }
                Op::Lte => {
                    if comp_reg <= inst.comp_imm {
                        dest_reg += delta
                    }
                }
                Op::Eq => {
                    if comp_reg == inst.comp_imm {
                        dest_reg += delta
                    }
                }
                Op::Ne => {
                    if comp_reg != inst.comp_imm {
                        dest_reg += delta
                    }
                }
            }

            if dest_reg > max {
                max = dest_reg;
            }
            self.registers[inst.dest_reg] = dest_reg;
        }

        (self.registers.iter().copied().max().unwrap(), max)
    }
}

impl Runner for Day08 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let mut registers = HashMap::default();
        macro_rules! register {
            ($name:expr) => {{
                let next = registers.len();
                *registers.entry($name).or_insert(next)
            }};
        }
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let inst = Instruction {
                dest_reg: register!(parts[0]),
                inc: parts[1] == "inc",
                delta: parts[2].parse()?,
                comp_reg: register!(parts[4]),
                comp_op: parts[5].parse()?,
                comp_imm: parts[6].parse()?,
            };
            self.instructions.push(inst);
        }
        (0..registers.len()).for_each(|_| self.registers.push(0));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.run().0.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.run().1.into())
    }
}
