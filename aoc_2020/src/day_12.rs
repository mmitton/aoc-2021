use std::str::FromStr;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn turn(&self, turns: isize) -> Self {
        let d = match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        };
        match (d + turns).rem_euclid(4) {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => unreachable!(),
        }
    }

    fn deltas(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        }
    }
}

#[derive(Debug)]
enum Inst {
    Left(isize),
    Right(isize),
    Dir(Dir, isize),
    Forward(isize),
}

impl FromStr for Inst {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: isize = s[1..].parse()?;
        match &s[0..1] {
            "N" => Ok(Self::Dir(Dir::North, num)),
            "S" => Ok(Self::Dir(Dir::South, num)),
            "E" => Ok(Self::Dir(Dir::East, num)),
            "W" => Ok(Self::Dir(Dir::West, num)),
            "L" => Ok(Self::Left(num / 90)),
            "R" => Ok(Self::Right(num / 90)),
            "F" => Ok(Self::Forward(num)),
            _ => Err(Error::Runner(format!("Invalid Instruction: '{}'", s))),
        }
    }
}

pub struct Day12 {
    insts: Vec<Inst>,
}

impl Day12 {
    pub fn new() -> Self {
        Self { insts: Vec::new() }
    }
}

impl Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.insts.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut dir = Dir::East;
        let mut pos: (isize, isize) = (0, 0);
        for inst in self.insts.iter() {
            match inst {
                Inst::Dir(dir, d) => {
                    let (dx, dy) = dir.deltas();
                    pos.0 += d * dx;
                    pos.1 += d * dy;
                }
                Inst::Left(d) => dir = dir.turn(-d),
                Inst::Right(d) => dir = dir.turn(*d),
                Inst::Forward(d) => {
                    let (dx, dy) = dir.deltas();
                    pos.0 += d * dx;
                    pos.1 += d * dy;
                }
            }
        }

        Ok((pos.0.abs() + pos.1.abs()).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        fn matrix(turns: isize, waypoint: (isize, isize)) -> (isize, isize) {
            let matrix = match turns.rem_euclid(4) {
                0 => [[1, 0], [0, 1]],
                1 => [[0, -1], [1, 0]],
                2 => [[-1, 0], [0, -1]],
                3 => [[0, 1], [-1, 0]],
                _ => unreachable!(),
            };
            let wx = (waypoint.0 * matrix[0][0]) + (waypoint.1 * matrix[0][1]);
            let wy = (waypoint.0 * matrix[1][0]) + (waypoint.1 * matrix[1][1]);
            (wx, wy)
        }

        let mut pos: (isize, isize) = (0, 0);
        let mut waypoint = (10, -1);
        for inst in self.insts.iter() {
            match inst {
                Inst::Dir(dir, d) => {
                    let (dx, dy) = dir.deltas();
                    waypoint.0 += d * dx;
                    waypoint.1 += d * dy;
                }
                Inst::Left(d) => {
                    waypoint = matrix(-d, waypoint);
                }
                Inst::Right(d) => {
                    waypoint = matrix(*d, waypoint);
                }
                Inst::Forward(d) => {
                    pos.0 += d * waypoint.0;
                    pos.1 += d * waypoint.1;
                }
            }
        }

        Ok((pos.0.abs() + pos.1.abs()).into())
    }
}
