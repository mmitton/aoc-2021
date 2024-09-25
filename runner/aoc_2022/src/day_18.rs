#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Point {
    x: i8,
    y: i8,
    z: i8,
}

pub struct Day18 {
    grid: HashSet<Point>,
    min: Point,
    max: Point,
}

impl Day18 {
    pub fn new() -> Self {
        Self {
            grid: HashSet::new(),
            min: Point {
                x: i8::MAX,
                y: i8::MAX,
                z: i8::MAX,
            },
            max: Point {
                x: i8::MIN,
                y: i8::MIN,
                z: i8::MIN,
            },
        }
    }
}

impl Runner for Day18 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let nums: Vec<&str> = line.split(',').collect();
            let x = nums[0].parse()?;
            let y = nums[1].parse()?;
            let z = nums[2].parse()?;

            self.min.x = self.min.x.min(x - 1);
            self.min.y = self.min.y.min(y - 1);
            self.min.z = self.min.z.min(z - 1);
            self.max.x = self.max.x.max(x + 1);
            self.max.y = self.max.y.max(y + 1);
            self.max.z = self.max.z.max(z + 1);

            self.grid.insert(Point { x, y, z });
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;
        for p in self.grid.iter() {
            if !self.grid.contains(&Point {
                x: p.x + 1,
                y: p.y,
                z: p.z,
            }) {
                ans += 1;
            }
            if !self.grid.contains(&Point {
                x: p.x - 1,
                y: p.y,
                z: p.z,
            }) {
                ans += 1;
            }
            if !self.grid.contains(&Point {
                x: p.x,
                y: p.y + 1,
                z: p.z,
            }) {
                ans += 1;
            }
            if !self.grid.contains(&Point {
                x: p.x,
                y: p.y - 1,
                z: p.z,
            }) {
                ans += 1;
            }
            if !self.grid.contains(&Point {
                x: p.x,
                y: p.y,
                z: p.z + 1,
            }) {
                ans += 1;
            }
            if !self.grid.contains(&Point {
                x: p.x,
                y: p.y,
                z: p.z - 1,
            }) {
                ans += 1;
            }
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;
        let mut work = vec![self.min];
        let mut seen = HashSet::new();
        seen.insert(self.min);
        let mut idx = 0;
        while idx < work.len() {
            let at = work[idx];
            idx += 1;

            macro_rules! process {
                ($x:expr, $y: expr, $z:expr) => {
                    let p = Point {
                        x: $x,
                        y: $y,
                        z: $z,
                    };

                    if !seen.contains(&p)
                        && p.x >= self.min.x
                        && p.x <= self.max.x
                        && p.y >= self.min.y
                        && p.y <= self.max.y
                        && p.z >= self.min.z
                        && p.z <= self.max.z
                    {
                        if self.grid.contains(&p) {
                            ans += 1;
                        } else {
                            work.push(p);
                            seen.insert(p);
                        }
                    }
                };
            }
            process!(at.x - 1, at.y, at.z);
            process!(at.x + 1, at.y, at.z);
            process!(at.x, at.y - 1, at.z);
            process!(at.x, at.y + 1, at.z);
            process!(at.x, at.y, at.z - 1);
            process!(at.x, at.y, at.z + 1);
        }

        Ok(ans.into())
    }
}
