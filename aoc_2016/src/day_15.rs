#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug)]
struct Disc {
    num: isize,
    positions: isize,
    t0: isize,
}

impl Disc {
    fn position_at(&self, t: isize) -> isize {
        (self.t0 + t + self.num) % self.positions
    }
}

impl FromStr for Disc {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(".", "");
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 12 {
            return Err(Error::InvalidInput(s.to_string()));
        }

        Ok(Self {
            num: parts[1][1..].parse()?,
            positions: parts[3].parse()?,
            t0: parts[11].parse()?,
        })
    }
}

#[derive(Default)]
pub struct Day15 {
    discs: Vec<Disc>,
}

impl Day15 {
    pub fn new() -> Self {
        Self::default()
    }

    fn solve(&self) -> isize {
        'drop_loop: for t in 0.. {
            for disc in self.discs.iter() {
                if disc.position_at(t) != 0 {
                    continue 'drop_loop;
                }
            }

            return t;
        }
        unreachable!();
    }
}

impl Runner for Day15 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.discs.push(line.parse()?);
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

impl Day15 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.solve().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.discs.push(Disc {
            num: self.discs.len() as isize + 1,
            positions: 11,
            t0: 0,
        });
        Ok(self.solve().into())
    }
}
