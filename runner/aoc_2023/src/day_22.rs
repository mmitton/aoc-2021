#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
    z: usize,
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 3 {
            return Err(Error::InvalidInput(format!("Point: '{s}'")));
        }
        let x = coords[0].parse()?;
        let y = coords[1].parse()?;
        let z = coords[2].parse()?;

        Ok(Point { x, y, z })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    start: Point,
    end: Point,
    min: Point,
    max: Point,
    holding_up: Vec<usize>,
    sitting_on: Vec<usize>,
    holding_up_fully: HashSet<usize>,
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min.cmp(&other.min)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();
        let start: Point = start.parse()?;
        let end: Point = end.parse()?;
        let (start, end) = if start < end {
            (start, end)
        } else {
            (end, start)
        };
        let min = Point {
            x: start.x.min(end.x),
            y: start.y.min(end.y),
            z: start.z.min(end.z),
        };
        let max = Point {
            x: start.x.max(end.x),
            y: start.y.max(end.y),
            z: start.z.max(end.z),
        };
        Ok(Self {
            start,
            end,
            min,
            max,
            holding_up: Vec::new(),
            sitting_on: Vec::new(),
            holding_up_fully: HashSet::new(),
        })
    }
}

impl Brick {
    fn overlaps_at(&self, other: &Self) -> Option<usize> {
        let sx = self.min.x.max(other.min.x);
        let sy = self.min.y.max(other.min.y);

        let ex = self.max.x.min(other.max.x);
        let ey = self.max.y.min(other.max.y);

        if sx > ex || sy > ey {
            return None;
        }

        if self.max.z < other.min.z {
            Some(self.max.z)
        } else {
            Some(other.max.z)
        }
    }
}

pub struct Day22 {
    bricks: Vec<Brick>,
}

impl Day22 {
    pub fn new() -> Self {
        Self { bricks: Vec::new() }
    }

    fn settle_bricks(&mut self) {
        self.bricks.sort();
        for i in 0..self.bricks.len() {
            let mut max_z = 0;
            for j in 0..i {
                if let Some(z) = self.bricks[i].overlaps_at(&self.bricks[j]) {
                    if z > max_z {
                        self.bricks[i].sitting_on.clear();
                        max_z = z;
                    }
                    if z == max_z {
                        self.bricks[i].sitting_on.push(j);
                    }
                }
            }
            for j in 0..self.bricks[i].sitting_on.len() {
                let idx = self.bricks[i].sitting_on[j];
                self.bricks[idx].holding_up.push(i);
            }
            max_z += 1;
            let dz = self.bricks[i].min.z - max_z;
            self.bricks[i].start.z -= dz;
            self.bricks[i].end.z -= dz;
            self.bricks[i].min.z -= dz;
            self.bricks[i].max.z -= dz;
        }

        for i in 0..self.bricks.len() {
            let mut holding_up = HashSet::new();
            holding_up.insert(i);
            loop {
                let mut added = false;
                for j in i..self.bricks.len() {
                    if self.bricks[j].sitting_on.is_empty() || holding_up.contains(&j) {
                        continue;
                    }
                    if self.bricks[j]
                        .sitting_on
                        .iter()
                        .filter(|so| holding_up.contains(so))
                        .count()
                        == self.bricks[j].sitting_on.len()
                    {
                        added = true;
                        holding_up.insert(j);
                    }
                }
                if !added {
                    break;
                }
            }

            holding_up.remove(&i);
            self.bricks[i].holding_up_fully = holding_up;
        }
    }
}

impl Runner for Day22 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        for line in Lines::from_path(path, LinesOpt::RAW)?.iter() {
            self.bricks.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.settle_bricks();
        Ok(self
            .bricks
            .iter()
            .filter(|b| b.holding_up_fully.is_empty())
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.settle_bricks();
        for (i, b) in self.bricks.iter().enumerate() {
            println!("{i} {:?}", b.holding_up_fully);
        }
        Ok(self
            .bricks
            .iter()
            .map(|b| b.holding_up_fully.len())
            .sum::<usize>()
            .into())
    }
}
