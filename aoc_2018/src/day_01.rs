#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day01 {
    changes: Vec<isize>,
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let line = if let Some(line) = line.strip_prefix('+') {
                line
            } else {
                line
            };
            self.changes.push(line.parse()?);
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

impl Day01 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.changes.iter().sum::<isize>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut seen = HashSet::default();
        let mut freq = 0;
        loop {
            for change in self.changes.iter() {
                if !seen.insert(freq) {
                    return Ok(freq.into());
                }
                freq += change;
            }
        }
    }
}
