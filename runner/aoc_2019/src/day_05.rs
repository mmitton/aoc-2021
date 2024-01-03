#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

use crate::intcode::IntCode;

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
        self.intcode.run();
        Ok(self.intcode.output.unwrap().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.intcode.input.push_front(5);
        self.intcode.run();
        Ok(self.intcode.output.unwrap().into())
    }
}
