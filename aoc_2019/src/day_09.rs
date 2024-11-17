use crate::intcode::{IntCode, State};
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day09 {
    intcode: IntCode<i128>,
}

impl Day09 {
    pub fn new() -> Self {
        Self {
            intcode: Default::default(),
        }
    }
}

impl Runner for Day09 {
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

impl Day09 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.intcode.input.push_front(1);
        let mut output = 0;
        loop {
            match self.intcode.run() {
                State::HasOutput(v) => output = v,
                State::Stopped => break,
                x => unreachable!("Unexpected state: {x:?}"),
            }
        }
        Ok((output as usize).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.intcode.input.push_front(2);
        let mut output = 0;
        loop {
            match self.intcode.run() {
                State::HasOutput(v) => output = v,
                State::Stopped => break,
                x => unreachable!("Unexpected state: {x:?}"),
            }
        }
        Ok((output as usize).into())
    }
}
