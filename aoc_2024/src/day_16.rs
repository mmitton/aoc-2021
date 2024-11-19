#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day16 {}

impl Day16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Err(Error::Unsolved)
    }
}

impl helper::Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let _lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
