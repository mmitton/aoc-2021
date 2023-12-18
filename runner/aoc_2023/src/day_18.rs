#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::{cmp::Ordering, ops::RangeInclusive, str::FromStr};

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
    dir: Dir,
    dist: isize,
    p2_dir: Dir,
    p2_dist: isize,
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
            dir,
            dist,
            p2_dir,
            p2_dist,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Line {
    horizontal: bool,
    p1: Point,
    p2: Point,
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.p1.x.cmp(&other.p1.x) {
            x @ Ordering::Less | x @ Ordering::Greater => return x,
            _ => {}
        }
        match self.horizontal.cmp(&other.horizontal) {
            x @ Ordering::Less | x @ Ordering::Greater => return x,
            _ => {}
        }
        self.p1.y.cmp(&other.p1.y)
    }
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Self {
            horizontal: p1.x != p2.x,
            p1,
            p2,
        }
    }
}

pub struct Day18 {
    instructions: Vec<Instruction>,
    min: Point,
    max: Point,
    lines: Vec<Line>,
}

impl Day18 {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            lines: Vec::new(),
            min: Point::new(0, 0),
            max: Point::new(0, 0),
        }
    }

    fn dig(&mut self) {
        let mut p1: Point = Point::new(0, 0);
        for inst in self.instructions.iter() {
            let (dx, dy) = match inst.dir {
                Dir::Up => (0, -1),
                Dir::Down => (0, 1),
                Dir::Left => (-1, 0),
                Dir::Right => (1, 0),
            };
            let p2 = Point::new(p1.x + dx * inst.dist, p1.y + dy * inst.dist);
            self.min.x = self.min.x.min(p2.x);
            self.max.x = self.max.x.max(p2.x);
            self.min.y = self.min.y.min(p2.y);
            self.max.y = self.max.y.max(p2.y);

            if p2 < p1 {
                self.lines.push(Line::new(p2, p1));
            } else {
                self.lines.push(Line::new(p1, p2));
            }
            p1 = p2;
        }
        self.lines.sort();
    }

    fn line(&self, y: isize, ranges: &mut Vec<RangeInclusive<isize>>) {
        ranges.clear();
        let mut crossed_at = None;
        let mut start = None;
        for line in self
            .lines
            .iter()
            .filter(|l| !l.horizontal && l.p1.y <= y && l.p2.y >= y)
        {
            if line.p1.y == y || line.p2.y == y {
                if let Some((first, dir)) = crossed_at.take() {
                    if dir != (line.p1.y < y) {
                        if let Some(start) = start.take() {
                            ranges.push(start..=line.p1.x);
                        } else {
                            start = Some(first);
                        }
                    } else if start.is_none() {
                        ranges.push(first..=line.p1.x);
                    }
                } else {
                    crossed_at = Some((line.p1.x, line.p1.y < y));
                }
            } else if let Some(start) = start.take() {
                ranges.push(start..=line.p1.x);
            } else {
                start = Some(line.p1.x);
            }
        }
    }

    fn area(&mut self) -> usize {
        let mut area = 0;
        let mut ranges = Vec::new();

        for y in self.min.y..=self.max.y {
            self.line(y, &mut ranges);
            let mut line_area = 0;
            for r in ranges.iter() {
                line_area += r.end() - r.start() + 1;
            }
            area += line_area;
        }
        area as usize
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
        self.dig();
        Ok(self.area().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.instructions.iter_mut().for_each(|inst| {
            inst.dir = inst.p2_dir;
            inst.dist = inst.p2_dist;
        });
        self.dig();
        Ok(self.area().into())
    }
}
