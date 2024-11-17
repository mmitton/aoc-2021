#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day18 {
    iters: usize,
    num_on: usize,
    light_on: Vec<Vec<bool>>,
    neighbors_on: Vec<Vec<u8>>,
}

impl Day18 {
    pub fn new() -> Self {
        Self::default()
    }

    fn next(&mut self, corners_on: bool) {
        let max_r = self.light_on.len() - 1;
        let max_c = self.light_on[0].len() - 1;
        for r in 0..=max_r {
            for c in 0..=max_c {
                self.neighbors_on[r][c] = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr < 0 || nc < 0 || nr > max_r as isize || nc > max_c as isize {
                            continue;
                        }

                        self.neighbors_on[r][c] += if self.light_on[nr as usize][nc as usize] {
                            1
                        } else {
                            0
                        };
                    }
                }
            }
        }
        self.num_on = 0;
        for r in 0..=max_r {
            for c in 0..=max_c {
                if (r == max_r || r == 0) && (c == max_c || c == 0) && corners_on {
                    self.num_on += 1;
                    self.light_on[r][c] = true;
                } else if self.light_on[r][c] {
                    // stays on with 2 or 3 of neighbors on
                    if self.neighbors_on[r][c] == 2 || self.neighbors_on[r][c] == 3 {
                        self.num_on += 1;
                        self.light_on[r][c] = true;
                    } else {
                        self.light_on[r][c] = false;
                    }
                } else {
                    // Turns on with 3 neighbors on
                    if self.neighbors_on[r][c] == 3 {
                        self.num_on += 1;
                        self.light_on[r][c] = true;
                    } else {
                        self.light_on[r][c] = false;
                    }
                }
            }
        }
    }
}

impl Runner for Day18 {
    fn parse(&mut self, file: &[u8], part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let mut light_on = Vec::new();
            let mut neighbors_on = Vec::new();
            for c in line.chars() {
                neighbors_on.push(0);
                if c == '.' {
                    light_on.push(false);
                } else if c == '#' {
                    light_on.push(true);
                }
            }

            self.light_on.push(light_on);
            self.neighbors_on.push(neighbors_on);
        }

        self.iters = if self.light_on.len() == 6 {
            if part == 1 {
                4
            } else {
                5
            }
        } else {
            100
        };
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

impl Day18 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        for _ in 0..self.iters {
            self.next(false);
        }

        Ok(self.num_on.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let max_r = self.light_on.len() - 1;
        let max_c = self.light_on[0].len() - 1;
        self.light_on[0][0] = true;
        self.light_on[0][max_c] = true;
        self.light_on[max_r][0] = true;
        self.light_on[max_r][max_c] = true;

        for _ in 0..self.iters {
            self.next(true);
        }

        Ok(self.num_on.into())
    }
}
