#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

<<<<<<< HEAD
#[derive(Default)]
=======
>>>>>>> fe77141e2313be3cb7b01b86947b4609e1232177
pub struct Day24 {}

impl Day24 {
    pub fn new() -> Self {
<<<<<<< HEAD
        Self::default()
=======
        Self {}
>>>>>>> fe77141e2313be3cb7b01b86947b4609e1232177
    }
}

impl Runner for Day24 {
<<<<<<< HEAD
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let _lines = Lines::from_bufread(file, LinesOpt::RAW)?;
=======
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let _lines = Lines::from_path(path, LinesOpt::RAW)?;
>>>>>>> fe77141e2313be3cb7b01b86947b4609e1232177
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }
}
