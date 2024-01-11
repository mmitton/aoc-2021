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
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        self.intcode.load(Lines::from_path(path, LinesOpt::RAW)?)
    }

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
