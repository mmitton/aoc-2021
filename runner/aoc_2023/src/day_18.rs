#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    parts: [(Dir, isize); 2],
}

impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("Invalid Dir: '{s}'")),
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid Instruction: '{s}')"));
        }

        let dir: Dir = parts[0].parse()?;
        let dist: isize = parts[1].parse().unwrap();
        let p2_hex = &parts[2][2..7];
        let p2_dist = isize::from_str_radix(p2_hex, 16).unwrap();
        let p2_dir = match &parts[2][7..8] {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            _ => return Err(format!("Invalid part2 dir '{s}'")),
        };

        Ok(Self {
            parts: [(dir, dist), (p2_dir, p2_dist)],
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

pub struct Day18 {
    instructions: Vec<Instruction>,
}

impl Day18 {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    fn area(&mut self, part: usize) -> isize {
        let mut p1 = Point::new(0, 0);

        let area = self.instructions.iter().fold(0, |area, inst| {
            let (dir, dist) = inst.parts[part - 1];
            let (dx, dy) = match dir {
                Dir::Up => (0, -1),
                Dir::Down => (0, 1),
                Dir::Left => (-1, 0),
                Dir::Right => (1, 0),
            };
            let p2 = Point::new(p1.x + dx * dist, p1.y + dy * dist);

            let cur_area = p1.x * p2.y - p1.y * p2.x + dist;
            p1 = p2;
            area + cur_area
        });

        area / 2 + 1
    }
}

impl Runner for Day18 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        for line in Lines::from_path(path, LinesOpt::RAW)?.iter() {
            self.instructions.push(line.parse().unwrap());
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.area(1).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.area(2).into())
    }
}
