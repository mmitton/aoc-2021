#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day20 {
    key: [bool; 512],
    grid: Vec<Vec<bool>>,
}

impl Day20 {
    pub fn new() -> Self {
        Self {
            key: [false; 512],
            grid: Vec::new(),
        }
    }

    fn run(&mut self, iters: usize) {
        let width = self.grid[0].len();
        let height = self.grid.len();
        let mut grid = vec![vec![false; width + (iters + 1) * 2]; height + (iters + 1) * 2];
        for (y, line) in self.grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                grid[y + iters + 1][x + iters + 1] = *c;
            }
        }
        self.grid = grid;

        let border = if self.key[0] {
            [false, true]
        } else {
            [false, false]
        };

        let mut new_grid = self.grid.clone();

        let width = self.grid[0].len() - 1;
        let height = self.grid.len() - 1;
        for iter in 0..iters {
            // Set border
            let border = border[iter % 2];
            for y in 1..height {
                self.grid[y][0] = border;
                self.grid[y][width] = border;
            }
            for x in 0..=width {
                self.grid[0][x] = border;
                self.grid[height][x] = border;
            }
            for (y, new_row) in new_grid.iter_mut().enumerate().take(height).skip(1) {
                for (x, c) in new_row.iter_mut().enumerate().take(width).skip(1) {
                    let mut idx = if self.grid[y - 1][x - 1] { 1 } else { 0 };
                    idx = (idx << 1) | if self.grid[y - 1][x] { 1 } else { 0 };
                    idx = (idx << 1) | if self.grid[y - 1][x + 1] { 1 } else { 0 };
                    idx = (idx << 1) | if self.grid[y][x - 1] { 1 } else { 0 };
                    idx = (idx << 1) | if self.grid[y][x] { 1 } else { 0 };
                    idx = (idx << 1) | if self.grid[y][x + 1] { 1 } else { 0 };
                    idx = (idx << 1) | if self.grid[y + 1][x - 1] { 1 } else { 0 };
                    idx = (idx << 1) | if self.grid[y + 1][x] { 1 } else { 0 };
                    idx = (idx << 1) | if self.grid[y + 1][x + 1] { 1 } else { 0 };

                    *c = self.key[idx];
                }
            }

            std::mem::swap(&mut new_grid, &mut self.grid);
        }
    }

    fn _print(&self) {
        for line in self.grid.iter() {
            for c in line.iter() {
                print!("{}", if *c { '#' } else { '.' });
            }
            println!();
        }
    }
}

impl Runner for Day20 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        lines[0]
            .chars()
            .enumerate()
            .for_each(|(i, c)| self.key[i] = c == '#');
        for line in lines[2..].iter() {
            self.grid.push(line.chars().map(|c| c == '#').collect());
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.run(2);
        Ok(self
            .grid
            .iter()
            .map(|c| c.iter().filter(|c| **c).count())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.run(50);
        Ok(self
            .grid
            .iter()
            .map(|c| c.iter().filter(|c| **c).count())
            .sum::<usize>()
            .into())
    }
}
