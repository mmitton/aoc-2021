#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day03 {
    map: BTreeMap<(isize, isize), char>,
}

impl Day03 {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn digit(&self, x: isize, y: isize) -> Option<usize> {
        if let Some(ch) = self.map.get(&(x, y)) {
            if ch.is_ascii_digit() {
                return Some(*ch as usize - '0' as usize);
            }
        }

        None
    }
}

impl Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        for (y, line) in Lines::from_bufread(file, LinesOpt::RAW)?.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                self.map.insert((x as isize, y as isize), ch);
            }
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day03 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut total = 0;
        let mut seen = BTreeSet::new();
        for ((sx, sy), ch) in self.map.iter() {
            if *ch != '.' && !ch.is_ascii_digit() {
                println!("found symbol '{ch}' at {sx},{sy}");
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let x = sx + dx;
                        let y = sy + dy;
                        if seen.contains(&(x, y)) {
                            continue;
                        }
                        seen.insert((x, y));
                        if let Some(mut num) = self.digit(x, y) {
                            // scan left
                            let mut lx = x;
                            while let Some(digit) = self.digit(lx - 1, y) {
                                lx -= 1;
                                seen.insert((lx, y));
                                num = digit;
                            }
                            // scan right
                            let mut rx = lx;
                            while let Some(digit) = self.digit(rx + 1, y) {
                                num = num * 10 + digit;
                                rx += 1;
                                seen.insert((rx, y));
                            }

                            println!("found {num} at {lx},{y}");
                            total += num;
                        }
                    }
                }
            }
        }
        Ok(total.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut total = 0;
        for ((sx, sy), ch) in self.map.iter() {
            if *ch == '*' {
                let mut seen = BTreeSet::new();
                let mut nums = Vec::new();
                println!("found symbol '{ch}' at {sx},{sy}");
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let x = sx + dx;
                        let y = sy + dy;
                        if seen.contains(&(x, y)) {
                            continue;
                        }
                        seen.insert((x, y));
                        if let Some(mut num) = self.digit(x, y) {
                            // scan left
                            let mut lx = x;
                            while let Some(digit) = self.digit(lx - 1, y) {
                                lx -= 1;
                                seen.insert((lx, y));
                                num = digit;
                            }
                            // scan right
                            let mut rx = lx;
                            while let Some(digit) = self.digit(rx + 1, y) {
                                num = num * 10 + digit;
                                rx += 1;
                                seen.insert((rx, y));
                            }

                            println!("found {num} at {lx},{y}");
                            nums.push(num);
                        }
                    }
                }
                if nums.len() == 2 {
                    let ratio = nums[0] * nums[1];
                    println!("Found a gear as {sx},{sy}  {nums:?}  ratio:{ratio}");
                    total += ratio;
                }
            }
        }
        Ok(total.into())
    }
}
