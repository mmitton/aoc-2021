use std::str::FromStr;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn scale(&self, shift: u32) -> Self {
        Self {
            x: self.x >> shift,
            y: self.y >> shift,
            z: self.z >> shift,
        }
    }

    fn manhatten_dist(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct NanoBot {
    center: Point,
    radius: isize,
}

impl FromStr for NanoBot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("pos=<") {
            if let Some((pos, r)) = s.split_once(">, r=") {
                let mut xyz = pos.split(',');
                let x = xyz.next().ok_or(Error::InvalidInput(s.into()))?.parse()?;
                let y = xyz.next().ok_or(Error::InvalidInput(s.into()))?.parse()?;
                let z = xyz.next().ok_or(Error::InvalidInput(s.into()))?.parse()?;

                let r = r.parse()?;
                return Ok(Self {
                    center: Point { x, y, z },
                    radius: r,
                });
            }
        }
        Err(Error::InvalidInput(s.into()))
    }
}

impl NanoBot {
    fn limits(&self) -> [Point; 6] {
        [
            Point::new(self.center.x - self.radius, self.center.y, self.center.z),
            Point::new(self.center.x + self.radius, self.center.y, self.center.z),
            Point::new(self.center.x, self.center.y - self.radius, self.center.z),
            Point::new(self.center.x, self.center.y + self.radius, self.center.z),
            Point::new(self.center.x, self.center.y, self.center.z - self.radius),
            Point::new(self.center.x, self.center.y, self.center.z + self.radius),
        ]
    }

    fn scale(&self, shift: u32) -> Self {
        let center = self.center.scale(shift);
        let mut radius = 0;
        for limit in self.limits().map(|p| p.scale(shift)) {
            radius = radius.max(center.manhatten_dist(&limit));
        }
        Self { center, radius }
    }

    fn can_see(&self, p: &Point) -> bool {
        self.center.manhatten_dist(p) <= self.radius
    }
}

#[derive(Default)]
pub struct Day23 {
    nanobots: Vec<NanoBot>,
}

impl Day23 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day23 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.nanobots.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let strongest =
            self.nanobots.iter().fold(
                &self.nanobots[0],
                |s, n| if n.radius > s.radius { n } else { s },
            );

        Ok(self
            .nanobots
            .iter()
            .filter(|n| strongest.center.manhatten_dist(&n.center) <= strongest.radius)
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut search_spaces: Vec<Point> = Vec::new();
        let mut next_search_spaces: Vec<Point> = Vec::new();
        let mut scaled_nanobots = Vec::with_capacity(self.nanobots.len());
        let mut seen: HashSet<Point> = HashSet::default();

        search_spaces.push(Point::new(0, 0, 0));
        for shift in (0..isize::BITS).rev() {
            scaled_nanobots.clear();
            scaled_nanobots.extend(self.nanobots.iter().map(|nanobot| nanobot.scale(shift)));

            let mut max_seen = 0;
            next_search_spaces.clear();
            seen.clear();
            for point in search_spaces.iter() {
                for z in (point.z * 2) - 1..=(point.z * 2) + 1 {
                    for y in (point.y * 2) - 1..=(point.y * 2) + 1 {
                        for x in (point.x * 2) - 1..=(point.x * 2) + 1 {
                            let point = Point::new(x, y, z);
                            if seen.insert(point) {
                                let mut seen = 0;
                                for nanobot in scaled_nanobots.iter() {
                                    if nanobot.can_see(&point) {
                                        seen += 1;
                                    }
                                }

                                if seen > max_seen {
                                    next_search_spaces.clear();
                                    max_seen = seen;
                                }
                                if seen == max_seen {
                                    next_search_spaces.push(point);
                                }
                            }
                        }
                    }
                }
            }

            std::mem::swap(&mut search_spaces, &mut next_search_spaces);
        }

        let origin = Point::new(0, 0, 0);
        let mut closest = isize::MAX;

        for point in search_spaces {
            let dist = origin.manhatten_dist(&point);
            if dist < closest {
                closest = dist;
            }
        }
        Ok(closest.into())
    }
}
