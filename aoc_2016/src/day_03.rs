#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day03 {
    list: Vec<Vec<usize>>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.list.push(
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            );
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

impl Day03 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut possible = 0;
        for nums in self.list.iter() {
            if nums[0] + nums[1] > nums[2]
                && nums[0] + nums[2] > nums[1]
                && nums[1] + nums[2] > nums[0]
            {
                possible += 1;
            }
        }
        Ok(possible.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut possible = 0;
        for nums in self.list.chunks(3) {
            for x in 0..3 {
                if nums[0][x] + nums[1][x] > nums[2][x]
                    && nums[0][x] + nums[2][x] > nums[1][x]
                    && nums[1][x] + nums[2][x] > nums[0][x]
                {
                    possible += 1;
                }
            }
        }
        Ok(possible.into())
    }
}
