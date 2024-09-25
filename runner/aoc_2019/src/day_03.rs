#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    p0: Point,
    min: Point,
    max: Point,
    dist: usize,
}

impl Line {
    fn new(p0: Point, p1: Point, dist: usize) -> Self {
        Self {
            p0,
            min: Point {
                x: p0.x.min(p1.x),
                y: p0.y.min(p1.y),
            },
            max: Point {
                x: p0.x.max(p1.x),
                y: p0.y.max(p1.y),
            },
            dist,
        }
    }
}

pub struct Day03 {
    wires: Vec<Vec<Line>>,
}

impl Day03 {
    pub fn new() -> Self {
        Self { wires: Vec::new() }
    }

    fn intersections(&self) -> HashMap<Point, (usize, usize)> {
        let mut intersections = HashMap::new();
        let mut d0 = 0;
        for l0 in self.wires[0].iter() {
            let mut d1 = 0;
            for l1 in self.wires[1].iter() {
                let min = Point {
                    x: l0.min.x.max(l1.min.x),
                    y: l0.min.y.max(l1.min.y),
                };
                let max = Point {
                    x: l0.max.x.min(l1.max.x),
                    y: l0.max.y.min(l1.max.y),
                };

                if min.x <= max.x && min.y <= max.y {
                    for y in min.y..=max.y {
                        for x in min.x..=max.x {
                            if x != 0 || y != 0 {
                                let d0 = d0 + ((l0.p0.x - x).abs() + (l0.p0.y - y).abs()) as usize;
                                let d1 = d1 + ((l1.p0.x - x).abs() + (l1.p0.y - y).abs()) as usize;
                                let intersection = intersections
                                    .entry(Point { x, y })
                                    .or_insert((usize::MAX, usize::MAX));
                                intersection.0 = intersection.0.min(d0);
                                intersection.1 = intersection.1.min(d1);
                            }
                        }
                    }
                }
                d1 += l1.dist;
            }
            d0 += l0.dist;
        }

        intersections
    }
}

impl Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
            let mut wire = Vec::new();

            let mut p0 = Point { x: 0, y: 0 };

            for inst in line.split(',') {
                let (dx, dy) = match &inst[..1] {
                    "U" => (0, -1),
                    "D" => (0, 1),
                    "L" => (-1, 0),
                    "R" => (1, 0),
                    c => return Err(Error::InvalidInput(format!("Direction: '{c}'"))),
                };
                let dist: isize = inst[1..].parse()?;
                let p1 = Point {
                    x: p0.x + (dx * dist),
                    y: p0.y + (dy * dist),
                };
                wire.push(Line::new(p0, p1, dist as usize));
                p0 = p1;
            }
            self.wires.push(wire);
        }

        if self.wires.len() != 2 {
            Err(Error::InvalidInput(format!(
                "Expected 2 wires, got {}",
                self.wires.len()
            )))
        } else {
            Ok(())
        }
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .intersections()
            .keys()
            .map(|p| p.x.abs() + p.y.abs())
            .min()
            .unwrap()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .intersections()
            .values()
            .map(|(d0, d1)| d0 + d1)
            .min()
            .unwrap()
            .into())
    }
}
