#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug)]
enum Dir {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Dir::N),
            "ne" => Ok(Dir::NE),
            "se" => Ok(Dir::SE),
            "s" => Ok(Dir::S),
            "sw" => Ok(Dir::SW),
            "nw" => Ok(Dir::NW),
            _ => Err(Error::InvalidInput(s.into())),
        }
    }
}

#[derive(Default)]
pub struct Day11 {
    directions: Vec<Dir>,
}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }

    fn steps_away(x: isize, y: isize) -> usize {
        let mut steps = 0;
        if x == 0 {
            steps += (y / 2).unsigned_abs();
        } else {
            steps += x.unsigned_abs();
            if x.abs() < y.abs() {
                // Move y "x" steps towards zero
                let y = y + x.abs() * if y.is_positive() { -1 } else { 1 };
                // And then move to y == 0
                steps += (y / 2).unsigned_abs();
            }
        }

        steps
    }

    fn follow_path(&self) -> (usize, usize) {
        let mut x = 0isize;
        let mut y = 0isize;
        let mut max_steps_away = 0usize;
        let mut steps_away = 0usize;
        for dir in self.directions.iter() {
            match dir {
                Dir::N => y -= 2,
                Dir::NE => {
                    y -= 1;
                    x += 1;
                }
                Dir::SE => {
                    y += 1;
                    x += 1;
                }
                Dir::S => y += 2,
                Dir::SW => {
                    y += 1;
                    x -= 1;
                }
                Dir::NW => {
                    y -= 1;
                    x -= 1;
                }
            }
            steps_away = Self::steps_away(x, y);
            max_steps_away = max_steps_away.max(steps_away);
        }

        (steps_away, max_steps_away)
    }
}

impl Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        for dir in lines[0].split(',') {
            self.directions.push(dir.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.follow_path().0.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.follow_path().1.into())
    }
}
