#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::{ops::AddAssign, str::FromStr};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point(isize, isize);

impl Point {
    fn step_to(self, to: Self) -> Self {
        let dx = if self.0 == to.0 {
            0
        } else if (to.0 - self.0).is_negative() {
            -1
        } else {
            1
        };

        let dy = if self.1 == to.1 {
            0
        } else if (to.1 - self.1).is_negative() {
            -1
        } else {
            1
        };

        Self(dx, dy)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

enum Op {
    Left(isize),
    Right(isize),
}

enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn walk(self, op: &Op, at: Point) -> (Self, Point) {
        match op {
            Op::Left(dist) => match self {
                Self::North => (Self::West, Point(at.0 - dist, at.1)),
                Self::South => (Self::East, Point(at.0 + dist, at.1)),
                Self::East => (Self::North, Point(at.0, at.1 - dist)),
                Self::West => (Self::South, Point(at.0, at.1 + dist)),
            },
            Op::Right(dist) => match self {
                Self::North => (Self::East, Point(at.0 + dist, at.1)),
                Self::South => (Self::West, Point(at.0 - dist, at.1)),
                Self::East => (Self::South, Point(at.0, at.1 + dist)),
                Self::West => (Self::North, Point(at.0, at.1 - dist)),
            },
        }
    }
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(dist) = s.strip_prefix("L") {
            Ok(Self::Left(dist.parse()?))
        } else if let Some(dist) = s.strip_prefix("R") {
            Ok(Self::Right(dist.parse()?))
        } else {
            Err(Error::InvalidInput(s.into()))
        }
    }
}

#[derive(Default)]
pub struct Day01 {
    ops: Vec<Op>,
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        assert_eq!(lines.len(), 1);
        for line in lines[0].split(", ") {
            self.ops.push(line.parse()?);
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
        let mut at = Point(0isize, 0isize);
        let mut dir = Dir::North;
        for op in self.ops.iter() {
            (dir, at) = dir.walk(op, at);
        }
        Ok((at.0.abs() + at.1.abs()).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut at = Point(0isize, 0isize);
        let mut dir = Dir::North;
        let mut seen = HashSet::default();
        let mut to: Point;
        seen.insert(at);
        for op in self.ops.iter() {
            (dir, to) = dir.walk(op, at);
            let step = at.step_to(to);
            while to != at {
                at += step;
                if !seen.insert(at) {
                    return Ok((at.0.abs() + at.1.abs()).into());
                }
            }
        }
        Err(Error::Unsolved)
    }
}
