#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum OpCode {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

struct Instruction {
    opcode: OpCode,
    a: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();

        let opcode = match parts[0] {
            "addr" => OpCode::AddR,
            "addi" => OpCode::AddI,
            "mulr" => OpCode::MulR,
            "muli" => OpCode::MulI,
            "banr" => OpCode::BanR,
            "bani" => OpCode::BanI,
            "borr" => OpCode::BorR,
            "bori" => OpCode::BorI,
            "setr" => OpCode::SetR,
            "seti" => OpCode::SetI,
            "gtir" => OpCode::GtIR,
            "gtri" => OpCode::GtRI,
            "gtrr" => OpCode::GtRR,
            "eqir" => OpCode::EqIR,
            "eqri" => OpCode::EqRI,
            "eqrr" => OpCode::EqRR,
            _ => return Err(Error::InvalidInput(s.into())),
        };
        let a = parts[1].parse()?;
        Ok(Self { opcode, a })
    }
}

#[derive(Default)]
pub struct Day21 {
    instructions: Vec<Instruction>,
}

impl Day21 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find(&self, first: bool) -> usize {
        let n: usize = 16777215;
        let a = if let Instruction {
            opcode: OpCode::SetI,
            a,
        } = self.instructions[7]
        {
            a
        } else {
            panic!("Could not find constant");
        };
        let x = 65899;
        let x2 = (x * x) & n;
        let x3 = (x2 * x) & n;

        let mut prev_r5;
        let mut r5 = 0;
        let mut seen = HashSet::default();
        loop {
            prev_r5 = r5;
            r5 = (((r5 >> 16) | 1) * x + ((r5 >> 8) & 255) * x2 + ((r5 & 255) + a) * x3) & n;
            if first && seen.is_empty() {
                return r5;
            }
            if !seen.insert(r5) {
                return prev_r5;
            }
        }
    }
}

impl Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if line.strip_prefix("#ip ").is_none() {
                self.instructions.push(line.parse()?);
            }
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

impl Day21 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find(true).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find(false).into())
    }
}
