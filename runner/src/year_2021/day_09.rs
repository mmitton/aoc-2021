use crate::{Error, Lines, LinesOpt, Output, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day09 {
    output: Output,
}

impl Day09 {
    pub fn new(part: usize) -> Self {
        Self { 
            output: Output::new(2021, 9, part),
        }
    }
}

impl Runner for Day09 {
    fn parse(&mut self, part: usize) -> Result<(), Error> {
        let _lines = Lines::find_day_part(&mut self.output, 2021, 9, part, LinesOpt::RAW)?;
        Ok(())
    }

    fn part1(&mut self) -> Result<(), Error> {
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<(), Error> {
        Err(Error::Unsolved)
    }

    fn output(&mut self) -> &mut Output {
        &mut self.output
    }
}
