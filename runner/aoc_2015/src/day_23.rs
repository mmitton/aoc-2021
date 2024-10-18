#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

enum Op {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn register(c: char) -> usize {
            (c as u8 - b'a') as usize
        }

        match &s[0..3] {
            "hlf" => Ok(Self::Hlf(register(s.chars().nth(4).unwrap()))),
            "tpl" => Ok(Self::Tpl(register(s.chars().nth(4).unwrap()))),
            "inc" => Ok(Self::Inc(register(s.chars().nth(4).unwrap()))),
            "jmp" => Ok(Self::Jmp(s[4..].parse()?)),
            "jie" => {
                if &s[7..8] == "+" {
                    Ok(Self::Jie(
                        register(s.chars().nth(4).unwrap()),
                        s[8..].parse()?,
                    ))
                } else {
                    Ok(Self::Jie(
                        register(s.chars().nth(4).unwrap()),
                        s[7..].parse()?,
                    ))
                }
            }
            "jio" => {
                if &s[7..8] == "+" {
                    Ok(Self::Jio(
                        register(s.chars().nth(4).unwrap()),
                        s[8..].parse()?,
                    ))
                } else {
                    Ok(Self::Jio(
                        register(s.chars().nth(4).unwrap()),
                        s[7..].parse()?,
                    ))
                }
            }
            _ => Err(Error::InvalidInput(s.to_string())),
        }
    }
}

#[derive(Default)]
pub struct Day23 {
    ops: Vec<Op>,
}

impl Day23 {
    pub fn new() -> Self {
        Self::default()
    }

    fn run(&self, a: usize) -> usize {
        let mut registers = [a, 0usize];
        let mut pc = 0isize;
        while let Some(op) = self.ops.get(pc as usize) {
            let mut next_pc = pc + 1;
            match op {
                Op::Hlf(r) => registers[*r] /= 2,
                Op::Tpl(r) => registers[*r] *= 3,
                Op::Inc(r) => registers[*r] += 1,
                Op::Jmp(offset) => next_pc = pc + offset,
                Op::Jie(r, offset) => {
                    if registers[*r] % 2 == 0 {
                        next_pc = pc + offset;
                    }
                }
                Op::Jio(r, offset) => {
                    if registers[*r] == 1 {
                        next_pc = pc + offset;
                    }
                }
            }
            pc = next_pc;
        }

        registers[1]
    }
}

impl Runner for Day23 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.ops.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.run(0).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.run(1).into())
    }
}
