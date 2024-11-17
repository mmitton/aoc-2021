#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day01 {
    data: Vec<u8>,
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }

    fn captcha(&self, offset: usize) -> usize {
        let mut sum = 0usize;
        for i in 0..self.data.len() {
            if self.data[i] == self.data[(i + offset) % self.data.len()] {
                sum += self.data[i] as usize;
            }
        }

        sum
    }
}

impl Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        for c in lines[0].chars() {
            self.data.push(c as u8 - b'0');
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
        Ok(self.captcha(1).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.captcha(self.data.len() / 2).into())
    }
}
