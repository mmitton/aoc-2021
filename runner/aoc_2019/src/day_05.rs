use crate::intcode::{IntCode, State};
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day05 {
    intcode: IntCode<i32>,
}

impl Day05 {
    pub fn new() -> Self {
        Self {
            intcode: IntCode::default(),
        }
    }
}

impl Runner for Day05 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        self.intcode.load(Lines::from_path(path, LinesOpt::RAW)?)
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.intcode.input.push_front(1);
        let mut output = 0;
        loop {
            match self.intcode.run() {
                State::HasOutput(v) => output = v,
                State::Running => {}
                State::Stopped => break,
                _ => unreachable!(),
            }
        }
        Ok(output.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.intcode.input.push_front(5);
        let mut output = 0;
        loop {
            match self.intcode.run() {
                State::HasOutput(v) => output = v,
                State::Running => {}
                State::Stopped => break,
                _ => unreachable!(),
            }
        }
        Ok(output.into())
    }
}
