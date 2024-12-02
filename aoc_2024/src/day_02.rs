#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};
use std::str::FromStr;

struct Level {
    reports: Vec<isize>,
}

impl Level {
    fn is_safe(&self, has_dampener: bool) -> bool {
        fn is_safe(mut iter: impl Iterator<Item = isize>) -> bool {
            let mut last = iter.next().unwrap();
            let mut is_increasing = false;
            for (i, next) in iter.enumerate() {
                if i == 0 {
                    is_increasing = last < next;
                }
                if (last < next) != is_increasing || !(1..=3).contains(&(next - last).abs()) {
                    return false;
                }

                last = next;
            }
            true
        }

        if is_safe(self.reports.iter().copied()) {
            return true;
        }
        if has_dampener {
            for i in 0..self.reports.len() {
                if is_safe(self.reports.iter().enumerate().filter_map(|(j, v)| {
                    if j != i {
                        Some(*v)
                    } else {
                        None
                    }
                })) {
                    return true;
                }
            }
        }
        false
    }
}

impl FromStr for Level {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reports = Vec::new();
        for num in s.split_whitespace() {
            reports.push(num.parse()?);
        }
        Ok(Self { reports })
    }
}

#[derive(Default)]
pub struct Day02 {
    levels: Vec<Level>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .levels
            .iter()
            .filter(|l| l.is_safe(false))
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .levels
            .iter()
            .filter(|l| l.is_safe(true))
            .count()
            .into())
    }
}

impl helper::Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.levels.push(line.parse()?);
        }
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
