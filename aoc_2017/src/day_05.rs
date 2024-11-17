#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day05 {
    jumps: Vec<isize>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn steps_to_exit(&mut self, stranger: bool) -> usize {
        let mut steps = 0;
        let mut pc = 0;
        while let Some(offset) = self.jumps.get_mut(pc as usize) {
            steps += 1;
            pc += *offset;
            if !stranger || *offset < 3 {
                *offset += 1;
            } else {
                *offset -= 1;
            }
        }

        steps
    }
}

impl Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.jumps.push(line.parse()?);
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

impl Day05 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.steps_to_exit(false).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.steps_to_exit(true).into())
    }
}
