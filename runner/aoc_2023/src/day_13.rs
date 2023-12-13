#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeSet;

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

struct Map {
    max: Point,
    rocks: BTreeSet<Point>,
}

impl Map {
    fn new() -> Self {
        Self {
            max: Point::new(0, 0),
            rocks: BTreeSet::new(),
        }
    }

    fn add_rock(&mut self, rock: Point) {
        self.max.x = self.max.x.max(rock.x);
        self.max.y = self.max.y.max(rock.y);
        self.rocks.insert(rock);
    }

    fn _dump(&self) {
        for y in 0..=self.max.y {
            for x in 0..=self.max.x {
                if self.rocks.contains(&Point::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn find_split(&self, dx: isize, dy: isize, diff: usize) -> Option<isize> {
        let mut x = 0;
        let mut y = 0;
        while x < self.max.x && y < self.max.y {
            x += dx;
            y += dy;

            // All rocks after the split line (but only enough rows/col to match the before depth)
            let rocks_after: Vec<Point> = self
                .rocks
                .iter()
                .filter(|r| {
                    (dx != 0 && r.x >= x && r.x < x * 2) || (dy != 0 && r.y >= y && r.y < y * 2)
                })
                .copied()
                .collect();

            // Mirror all of the rocks before the split line
            let rocks_before_mirrored: Vec<Point> = self
                .rocks
                .iter()
                .filter_map(|r| {
                    if (dx != 0 && r.x < x) || (dy != 0 && r.y < y) {
                        let rx = if dx != 0 { 2 * (x - 1) - r.x + 1 } else { r.x };
                        let ry = if dy != 0 { 2 * (y - 1) - r.y + 1 } else { r.y };
                        if rx <= self.max.x && ry <= self.max.y {
                            return Some(Point::new(rx, ry));
                        }
                    }
                    None
                })
                .collect();

            if rocks_after
                .iter()
                .filter(|r| !rocks_before_mirrored.contains(r))
                .count()
                + rocks_before_mirrored
                    .iter()
                    .filter(|r| !rocks_after.contains(r))
                    .count()
                == diff
            {
                return Some(x + y);
            }
        }

        None
    }
}

pub struct Day13 {
    maps: Vec<Map>,
}

impl Day13 {
    pub fn new() -> Self {
        Self { maps: Vec::new() }
    }
}

impl Runner for Day13 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let mut map = Map::new();
        let mut y = 0;
        for line in Lines::from_path(path, LinesOpt::RAW)?.iter() {
            if line.is_empty() {
                if !map.rocks.is_empty() {
                    self.maps.push(map);
                    map = Map::new();
                    y = 0;
                }
            } else {
                for (x, c) in line.chars().enumerate() {
                    match c {
                        '#' => map.add_rock(Point::new(x as isize, y)),
                        '.' => {}
                        _ => unreachable!("Wrong map char '{c}'"),
                    }
                }
                y += 1;
            }
        }
        if !map.rocks.is_empty() {
            self.maps.push(map);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .maps
            .iter()
            .map(|map| {
                map._dump();
                let mut ans = 0;
                if let Some(x_split) = map.find_split(1, 0, 0) {
                    print!("x_split: {x_split}  ");
                    ans += x_split;
                }
                if let Some(y_split) = map.find_split(0, 1, 0) {
                    print!("y_split: {y_split}  ");
                    ans += 100 * y_split;
                }
                println!("ans: {ans}");
                ans
            })
            .sum::<isize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .maps
            .iter()
            .map(|map| {
                map._dump();
                let mut ans = 0;
                if let Some(x_split) = map.find_split(1, 0, 1) {
                    print!("x_split: {x_split}  ");
                    ans += x_split;
                }
                if let Some(y_split) = map.find_split(0, 1, 1) {
                    print!("y_split: {y_split}  ");
                    ans += 100 * y_split;
                }
                println!("ans: {ans}");
                ans
            })
            .sum::<isize>()
            .into())
    }
}
