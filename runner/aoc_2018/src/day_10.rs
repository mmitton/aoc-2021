#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

type RawPoint = helper::Point<isize>;

#[derive(Debug)]
struct Point {
    p: RawPoint,
    v: RawPoint,
}

impl Point {
    fn at(&self, t: isize) -> RawPoint {
        RawPoint::new(self.p.x + self.v.x * t, self.p.y + self.v.y * t)
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("position=<") {
            let s = s.trim();
            if let Some((p, rest)) = s.split_once("> velocity=<") {
                if let Some(v) = rest.strip_suffix('>') {
                    fn point(s: &str) -> Result<RawPoint, Error> {
                        if let Some((x, y)) = s.split_once(',') {
                            Ok(RawPoint::new(x.trim().parse()?, y.trim().parse()?))
                        } else {
                            Err(Error::InvalidInput(s.into()))
                        }
                    }
                    return Ok(Self {
                        p: point(p)?,
                        v: point(v)?,
                    });
                }
            }
        }

        Err(Error::InvalidInput(s.into()))
    }
}

#[derive(Default)]
pub struct Day10 {
    points: Vec<Point>,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_area(&self, t: isize) -> (RawPoint, RawPoint, isize) {
        let mut min = RawPoint::new(isize::MAX, isize::MAX);
        let mut max = RawPoint::new(isize::MIN, isize::MIN);

        for p in self.points.iter().map(|p| p.at(t)) {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }

        (min, max, (max.x - min.x) * (max.y - min.y))
    }

    fn get_message(&self, t: isize) -> String {
        use std::collections::BTreeSet;

        let mut min = RawPoint::new(isize::MAX, isize::MAX);
        let mut max = RawPoint::new(isize::MIN, isize::MIN);
        let mut new_points = BTreeSet::new();

        for p in self.points.iter().map(|p| p.at(t)) {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            new_points.insert((p.x, p.y));
        }

        let mut message = String::new();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if new_points.contains(&(x, y)) {
                    message.push('#');
                } else {
                    message.push(' ');
                }
            }
            if y != max.y {
                message.push('\n');
            }
        }
        message
    }

    fn find_message_time(&self) -> isize {
        let mut area = isize::MAX;
        for t in 0.. {
            let (_, _, t_area) = self.get_area(t);
            if t_area > area {
                return t - 1;
            } else {
                area = t_area;
            }
        }
        unreachable!();
    }
}

impl Runner for Day10 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.points.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let t = self.find_message_time();
        Ok(self.get_message(t).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_message_time().into())
    }
}
