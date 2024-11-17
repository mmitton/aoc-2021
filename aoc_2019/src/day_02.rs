use crate::intcode::IntCode;
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day02 {
    intcode: IntCode<u32>,
}

impl Day02 {
    pub fn new() -> Self {
        Self {
            intcode: IntCode::default(),
        }
    }
}

impl Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        self.intcode.load(Lines::from_bufread(file, LinesOpt::RAW)?)
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day02 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.intcode[1] = 12;
        self.intcode[2] = 2;
        self.intcode.run();
        Ok(self.intcode[0].into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        if self.intcode.len() == 37 {
            return Err(Error::Skipped);
        }
        for a in 0..100 {
            for b in 0..100 {
                let mut intcode = self.intcode.clone();
                intcode[1] = a;
                intcode[2] = b;
                intcode.run();
                if intcode[0] == 19690720 {
                    return Ok(((a * 100) + b).into());
                }
            }
        }
        Err(Error::Unsolved)
    }
}
