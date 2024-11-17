use std::str::FromStr;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

struct Present {
    dimensions: [usize; 3],
}

impl Present {
    fn wrapping_paper_needed(&self) -> usize {
        let sides = [
            self.dimensions[0] * self.dimensions[1],
            self.dimensions[0] * self.dimensions[2],
            self.dimensions[1] * self.dimensions[2],
        ];
        let coverage = sides.iter().map(|l| 2 * l).sum::<usize>();
        let slack = sides.iter().min().unwrap();
        coverage + slack
    }

    fn ribbon_needed(&self) -> usize {
        let volume = self.dimensions.iter().product::<usize>();
        let min_perimeter = self.dimensions.iter().take(2).map(|l| 2 * l).sum::<usize>();

        min_perimeter + volume
    }
}

impl FromStr for Present {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 3 {
            return Err(Error::InvalidInput(s.into()));
        }
        let mut nums = Vec::with_capacity(3);
        for part in parts.iter() {
            nums.push(part.parse()?);
        }
        nums.sort();

        Ok(Self {
            dimensions: [nums[0], nums[1], nums[2]],
        })
    }
}

#[derive(Default)]
pub struct Day02 {
    presents: Vec<Present>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let present: Present = line.parse()?;
            self.presents.push(present);
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

impl Day02 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .presents
            .iter()
            .map(|p| p.wrapping_paper_needed())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .presents
            .iter()
            .map(|p| p.ribbon_needed())
            .sum::<usize>()
            .into())
    }
}
