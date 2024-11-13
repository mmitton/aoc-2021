#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day11 {
    grid: Vec<u8>,
}

impl Day11 {
    pub fn new() -> Self {
        Self {
            grid: vec![!0; 12 * 12],
        }
    }

    fn tick(&mut self) -> usize {
        macro_rules! inc {
            ($idx:expr) => {
                if self.grid[$idx] < 10 {
                    self.grid[$idx] += 1;
                }
            };
        }
        for y in 1..11 {
            for x in 1..11 {
                let idx = y * 12 + x;
                if self.grid[idx] == 11 {
                    self.grid[idx] = 1;
                } else {
                    self.grid[idx] += 1;
                }
            }
        }
        let mut flashes = 0;
        loop {
            let mut flashed = false;
            for y in 1..11 {
                for x in 1..11 {
                    let idx = y * 12 + x;
                    if self.grid[idx] == 10 {
                        self.grid[idx] += 1;
                        flashed = true;
                        flashes += 1;

                        inc!(idx - 13);
                        inc!(idx - 12);
                        inc!(idx - 11);
                        inc!(idx - 1);
                        inc!(idx + 1);
                        inc!(idx + 11);
                        inc!(idx + 12);
                        inc!(idx + 13);
                    }
                }
            }
            if !flashed {
                break;
            }
        }
        flashes
    }
}

impl Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 10);
        for (y, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), 10);
            for (x, c) in line.chars().enumerate() {
                let idx = (y + 1) * 12 + (x + 1);
                self.grid[idx] = c as u8 - b'0';
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok((0..100).map(|_| self.tick()).sum::<usize>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for i in 1.. {
            if self.tick() == 100 {
                return Ok(i.into());
            }
        }
        Err(Error::Unsolved)
    }
}
