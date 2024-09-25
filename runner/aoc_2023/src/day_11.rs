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

pub struct Day11 {
    galaxies: Vec<Point>,
    min: Point,
    max: Point,
}

impl Day11 {
    pub fn new() -> Self {
        Self {
            galaxies: Vec::new(),
            min: Point {
                x: isize::MAX,
                y: isize::MAX,
            },
            max: Point {
                x: isize::MIN,
                y: isize::MIN,
            },
        }
    }

    fn expand(&mut self, offset: isize) {
        let mut seen_x: BTreeSet<isize> = BTreeSet::new();
        let mut seen_y: BTreeSet<isize> = BTreeSet::new();
        for galaxy in self.galaxies.iter() {
            println!("galaxy: {galaxy:?}");

            seen_x.insert(galaxy.x);
            seen_y.insert(galaxy.y);
        }

        println!("min: {:?}", self.min);
        println!("max: {:?}", self.max);

        // Expand in the y direction
        for y in (self.min.y + 1..self.max.y).rev() {
            if !seen_y.contains(&y) {
                self.galaxies
                    .iter_mut()
                    .filter(|galaxy| galaxy.y > y)
                    .for_each(|galaxy| galaxy.y += offset - 1);
                self.max.y += offset - 1;
            }
        }

        // Expand in the x direction
        for x in (self.min.x + 1..self.max.x).rev() {
            if !seen_x.contains(&x) {
                self.galaxies
                    .iter_mut()
                    .filter(|galaxy| galaxy.x > x)
                    .for_each(|galaxy| galaxy.x += offset - 1);
                self.max.x += offset - 1;
            }
        }

        println!("expanded min: {:?}", self.min);
        println!("expanded max: {:?}", self.max);
    }

    fn distance_sum(&self) -> isize {
        let mut sum = 0;
        for (i1, g1) in self.galaxies.iter().enumerate() {
            for (i2, g2) in self.galaxies.iter().enumerate().skip(i1 + 1) {
                let dist = (g1.x - g2.x).abs() + (g1.y - g2.y).abs();
                sum += dist;
                println!("{i1} => {i2}  dist: {dist}  sum: {sum}");
            }
        }
        sum
    }

    fn _dump(&self) {
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                if self.galaxies.contains(&Point { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        for (y, line) in Lines::from_bufread(file, LinesOpt::RAW)?.iter().enumerate() {
            let y = y as isize;
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                match c {
                    '#' => {
                        self.galaxies.push(Point { x, y });
                        self.min.x = self.min.x.min(x);
                        self.min.y = self.min.y.min(y);
                        self.max.x = self.max.x.max(x);
                        self.max.y = self.max.y.max(y);
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.expand(2);
        Ok(self.distance_sum().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.expand(1000000);
        Ok(self.distance_sum().into())
    }
}
