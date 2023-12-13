#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

struct Map {
    pattern: Vec<usize>,
    pattern_rotated: Vec<usize>,
}

impl Map {
    fn new() -> Self {
        Self {
            pattern: Vec::new(),
            pattern_rotated: Vec::new(),
        }
    }

    fn rotate_pattern(&mut self, width: usize) {
        for bit in (0..=width).rev() {
            let pattern = self
                .pattern
                .iter()
                .rev()
                .enumerate()
                .fold(0, |r, (i, p)| r | ((p >> bit) & 1) << i);
            self.pattern_rotated.push(pattern);
        }
    }

    fn find_split(&self, pattern: &[usize], diff: usize) -> Option<usize> {
        for i in 1..pattern.len() {
            // Split the pattern at i
            let (a, b) = pattern.split_at(i);
            // Reverse a, zip it with b, and then count the total number of different bits (rocks)
            // Example:
            // p0, p1, p2, p3, p4
            // Split at i = 2
            // a = [p0, p1]
            // b = [p2, p3, p4]
            // Reverse a and zip with b gives
            // (p1, p2), (p0, p3) and no more since zip stops when either iterator returns None
            // This way we don't go off the end in either direction
            let mirrored_diff = a
                .iter()
                .rev()
                .zip(b.iter())
                .map(|(a, b)| (a ^ b).count_ones() as usize)
                .sum::<usize>();

            // If the total number of different rocks == number of different rocks expected (0 for
            // perfect match, 1 for smudged), return the index of the split
            if diff == mirrored_diff {
                return Some(i);
            }
        }
        None
    }

    fn get_mirror_ans(&self, diff: usize) -> usize {
        let mut ans = 0;
        if let Some(x_split) = self.find_split(&self.pattern_rotated, diff) {
            print!("x_split: {x_split}  ");
            ans += x_split;
        }
        if let Some(y_split) = self.find_split(&self.pattern, diff) {
            print!("y_split: {y_split}  ");
            ans += 100 * y_split;
        }
        println!("ans: {ans}");
        ans
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
        let mut w = 0;
        for line in Lines::from_path(path, LinesOpt::RAW)?.iter() {
            if line.is_empty() {
                if !map.pattern.is_empty() {
                    map.rotate_pattern(w);
                    self.maps.push(map);
                    map = Map::new();
                    w = 0;
                }
            } else {
                w = line.len() - 1;
                let mut pattern = 0;
                for c in line.chars() {
                    pattern <<= 1;
                    match c {
                        '#' => pattern |= 1,
                        '.' => {}
                        _ => unreachable!("Wrong map char '{c}'"),
                    }
                }
                map.pattern.push(pattern);
            }
        }
        if !map.pattern.is_empty() {
            map.rotate_pattern(w);
            self.maps.push(map);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .maps
            .iter()
            .map(|map| map.get_mirror_ans(0))
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .maps
            .iter()
            .map(|map| map.get_mirror_ans(1))
            .sum::<usize>()
            .into())
    }
}
