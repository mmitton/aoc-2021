#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, Point, RunOutput, Runner,
};
use std::cmp::Ordering;

#[derive(Default)]
pub struct Day06 {
    points: Vec<Point<isize>>,
    min: Point<isize>,
    max: Point<isize>,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_closest(&self, p: &Point<isize>) -> Option<usize> {
        let (i, _, n) = self.points.iter().enumerate().fold(
            (usize::MAX, isize::MAX, 0),
            |(c, d, n), (i, p1)| {
                let d1 = p1.manhattan_dist(p);
                match d1.cmp(&d) {
                    Ordering::Less => (i, d1, 1),
                    Ordering::Equal => (i, d1, n + 1),
                    Ordering::Greater => (c, d, n),
                }
            },
        );
        if n == 1 {
            Some(i)
        } else {
            None
        }
    }

    fn find_infinite(&self) -> Vec<usize> {
        let mut infinite = HashSet::default();

        let dx = 2 * (self.max.x - self.min.x);
        let dy = 2 * (self.max.y - self.min.y);

        for x in self.min.x..=self.max.x {
            for y in [self.min.y - dy, self.max.x + dy] {
                if let Some(closest) = self.find_closest(&Point::new(x, y)) {
                    infinite.insert(closest);
                }
            }
        }
        for y in self.min.y..=self.max.y {
            for x in [self.min.x - dx, self.max.x + dx] {
                if let Some(closest) = self.find_closest(&Point::new(x, y)) {
                    infinite.insert(closest);
                }
            }
        }

        let mut infinite: Vec<usize> = infinite.iter().copied().collect();
        infinite.sort();
        infinite
    }
}

impl Runner for Day06 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some((x, y)) = line.split_once(", ") {
                self.points.push(Point::new(x.parse()?, y.parse()?));
            }
        }

        self.min.x = isize::MAX;
        self.min.y = isize::MAX;
        self.max.x = isize::MIN;
        self.max.y = isize::MIN;

        for point in self.points.iter() {
            self.min.x = self.min.x.min(point.x);
            self.min.y = self.min.y.min(point.y);
            self.max.x = self.max.x.max(point.x);
            self.max.y = self.max.y.max(point.y);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let infinite = self.find_infinite();
        let mut regions = vec![0; self.points.len()];
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                if let Some(closest) = self.find_closest(&Point::new(x, y)) {
                    if !infinite.contains(&closest) {
                        regions[closest] += 1;
                    }
                }
            }
        }
        Ok(regions.iter().copied().max().unwrap().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let max_dist = if self.points.len() == 6 { 32 } else { 10000 };
        let mut regions = 0;
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                let p0 = Point::new(x, y);
                let sum_dist = self
                    .points
                    .iter()
                    .map(|p| p0.manhattan_dist(p))
                    .sum::<isize>();
                if sum_dist < max_dist {
                    regions += 1;
                }
            }
        }

        Ok(regions.into())
    }
}
