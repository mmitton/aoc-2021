#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day01 {
    depths: Vec<usize>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { depths: Vec::new() }
    }
}

impl Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.depths
            .extend(lines.iter().map(|l| l.parse::<usize>().unwrap()));
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
        Ok(self
            .depths
            .windows(2)
            .filter(|d| d[0] < d[1])
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let sums = self
            .depths
            .windows(3)
            .map(|d| d.iter().sum::<usize>())
            .collect::<Vec<usize>>();
        Ok(sums.windows(2).filter(|d| d[0] < d[1]).count().into())
    }
}
