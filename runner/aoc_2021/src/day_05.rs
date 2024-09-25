use std::str::FromStr;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point(i16, i16);

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self(x.parse()?, y.parse()?))
    }
}

pub struct Day05 {
    lines: Vec<(Point, Point)>,
}

impl Day05 {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn plot<F>(&self, filter: F) -> usize
    where
        F: Fn(&Point, &Point) -> bool,
    {
        let mut sums: HashMap<Point, usize> = HashMap::default();
        for (start, end) in self.lines.iter() {
            if filter(start, end) {
                fn delta(end: i16, start: i16) -> i16 {
                    use std::cmp::Ordering;
                    match end.cmp(&start) {
                        Ordering::Equal => 0,
                        Ordering::Less => -1,
                        Ordering::Greater => 1,
                    }
                }

                let dx = delta(end.0, start.0);
                let dy = delta(end.1, start.1);

                let mut p = *start;
                *sums.entry(p).or_default() += 1;
                while p != *end {
                    p.0 += dx;
                    p.1 += dy;
                    *sums.entry(p).or_default() += 1;
                }
            }
        }

        sums.values().filter(|s| **s > 1).count()
    }
}

impl Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        self.lines.extend(lines.iter().map(|l| {
            let (p1, p2) = l.split_once(" -> ").unwrap();
            (p1.parse().unwrap(), p2.parse().unwrap())
        }));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .plot(|start, end| start.0 == end.0 || start.1 == end.1)
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.plot(|_, _| true).into())
    }
}
