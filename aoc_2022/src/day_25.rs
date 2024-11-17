use std::str::FromStr;

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

struct Snafu(isize);

impl FromStr for Snafu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n: isize = 0;
        for c in s.chars() {
            n *= 5;
            match c {
                '0'..='2' => n += (c as u32 - '0' as u32) as isize,
                '-' => n -= 1,
                '=' => n -= 2,
                _ => unreachable!(),
            }
        }

        Ok(Self(n))
    }
}

impl std::fmt::Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut units: Vec<i8> = Vec::new();
        let mut n = self.0;
        while n != 0 {
            let rem = n % 5;
            n /= 5;

            units.push(rem as i8);
        }

        for idx in 0..units.len() {
            while units[idx] > 4 {
                units[idx] -= 5;
                if idx != units.len() - 1 {
                    units[idx + 1] += 1;
                } else {
                    units.push(1);
                }
            }
            if units[idx] > 2 {
                let d = 5 - units[idx];
                units[idx] = 0 - d;
                if idx != units.len() - 1 {
                    units[idx + 1] += 1;
                } else {
                    units.push(1);
                }
            }
        }

        for u in units.iter().rev() {
            match u {
                0..=2 => write!(f, "{u}")?,
                -1 => write!(f, "-")?,
                -2 => write!(f, "=")?,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl Snafu {
    fn new(n: isize) -> Self {
        Self(n)
    }
}

pub struct Day25 {
    nums: Vec<Snafu>,
}

impl Day25 {
    pub fn new() -> Self {
        Self { nums: Vec::new() }
    }
}

impl Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.nums.extend(lines.iter().map(|l| l.parse().unwrap()));
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day25 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let total: isize = self.nums.iter().map(|s| s.0).sum();
        Ok(Snafu::new(total).to_string().into())
    }
}
