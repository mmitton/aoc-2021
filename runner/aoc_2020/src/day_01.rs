#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day01 {
    nums: Vec<usize>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { nums: Vec::new() }
    }
}

impl Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.nums
            .extend(lines.iter().map(|l| l.parse::<usize>().unwrap()));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        for (i, a) in self.nums.iter().enumerate() {
            for b in self.nums[i + 1..].iter() {
                if a + b == 2020 {
                    return Ok((a * b).into());
                }
            }
        }
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for (i, a) in self.nums.iter().enumerate() {
            for (j, b) in self.nums[i + 1..].iter().enumerate() {
                for c in self.nums[j + 1..].iter() {
                    if a + b + c == 2020 {
                        return Ok((a * b * c).into());
                    }
                }
            }
        }
        Err(Error::Unsolved)
    }
}
