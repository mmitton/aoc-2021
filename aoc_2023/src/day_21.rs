#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{HashMap, HashSet};

type Number = i16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: Number,
    y: Number,
}

impl Point {
    fn new(x: Number, y: Number) -> Self {
        Self { x, y }
    }

    fn delta(&self, dx: Number, dy: Number, max: Self) -> (Self, Self) {
        (
            Self {
                x: self.x + dx,
                y: self.y + dy,
            },
            Self {
                x: (self.x + dx).rem_euclid(max.x),
                y: (self.y + dy).rem_euclid(max.y),
            },
        )
    }

    fn neighbors(&self, max: Self) -> [(Self, Self); 4] {
        [
            self.delta(1, 0, max),
            self.delta(-1, 0, max),
            self.delta(0, 1, max),
            self.delta(0, -1, max),
        ]
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    Plot,
    Rock,
    Start,
}

impl TryFrom<char> for Type {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Plot),
            '#' => Ok(Self::Rock),
            'S' => Ok(Self::Start),
            _ => Err(Error::InvalidInput(format!("Invalid Type: '{value}'"))),
        }
    }
}

pub struct Day21 {
    points: HashMap<Point, Type>,
    max: Point,
}

impl Day21 {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
            max: Point::new(0, 0),
        }
    }

    fn take_steps(&self, steps: &[usize]) -> Vec<usize> {
        let (start, _) = self
            .points
            .iter()
            .find(|(_, &typ)| typ == Type::Start)
            .unwrap();

        let mut prev_pos: HashSet<Point> = HashSet::new();
        let mut cur_pos: HashSet<Point> = HashSet::new();
        let mut next_pos: HashSet<Point> = HashSet::new();
        let mut prev_count = 0;
        let mut cur_count = 1;

        let mut start_offsets = HashSet::new();
        start_offsets.insert(Point::new(0, 0));
        cur_pos.insert(*start);
        let max_steps = *steps.iter().max().unwrap();
        let mut results = vec![0; steps.len()];

        for step in 1..=max_steps {
            next_pos.clear();
            for p in cur_pos.iter() {
                for (np, real_np) in p.neighbors(self.max) {
                    if self.points.contains_key(&real_np) && !prev_pos.contains(&np) {
                        next_pos.insert(np);
                    }
                }
            }

            let next_count = next_pos.len() + prev_count;
            if let Some(i) = steps.iter().position(|s| *s == step) {
                results[i] = next_count;
            }
            prev_count = cur_count;
            cur_count = next_count;
            std::mem::swap(&mut prev_pos, &mut cur_pos);
            std::mem::swap(&mut cur_pos, &mut next_pos);
        }

        results
    }
}

impl Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        for (y, line) in Lines::from_bufread(file, LinesOpt::RAW)?.iter().enumerate() {
            self.max.y = y as Number + 1;
            self.max.x = line.len() as Number;
            for (x, c) in line.chars().enumerate() {
                let typ: Type = c.try_into()?;
                if matches!(typ, Type::Plot | Type::Start) {
                    let p = Point::new(x as Number, y as Number);
                    self.points.insert(p, typ);
                }
            }
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let steps = if self.points.len() == 81 { 6 } else { 64 };
        Ok(self.take_steps(&[steps])[0].into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        if self.points.len() == 81 {
            return Ok(self.take_steps(&[100])[0].into());
        }

        let n = 26501365 / self.max.x as usize;

        let s1 = self.max.x as usize / 2;
        let s2 = s1 + self.max.x as usize;
        let s3 = s1 + self.max.x as usize * 2;
        let a = self.take_steps(&[s1, s2, s3]);
        println!("{a:?}");

        let b0 = a[0];
        let b1 = a[1] - a[0];
        let b2 = a[2] - a[1];

        Ok((b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1)).into())
    }
}
