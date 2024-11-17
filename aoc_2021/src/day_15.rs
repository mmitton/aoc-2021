use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day15 {
    grid: Vec<Vec<u8>>,
}

impl Day15 {
    pub fn new() -> Self {
        Self { grid: Vec::new() }
    }

    fn find_path(&self) -> usize {
        let mut queue: BTreeMap<usize, HashSet<(usize, usize)>> = BTreeMap::new();
        let mut lowest_cost: Vec<Vec<usize>> = self
            .grid
            .iter()
            .map(|g| g.iter().map(|_| usize::MAX).collect())
            .collect();
        lowest_cost[0][0] = 0;
        let mut initial = HashSet::default();
        initial.insert((0, 0));
        queue.insert(0, initial);
        let max_x = self.grid[0].len() - 1;
        let max_y = self.grid.len() - 1;
        while let Some((cost, work_queue)) = queue.pop_first() {
            for at in work_queue {
                macro_rules! walk {
                    ($x:expr, $y:expr) => {{
                        let x = $x;
                        let y = $y;
                        let next_cost = cost + self.grid[y][x] as usize;
                        if next_cost <= lowest_cost[y][x] {
                            if x == max_x && y == max_y {
                                return next_cost;
                            }
                            queue.entry(next_cost).or_default().insert((x, y));
                            lowest_cost[y][x] = next_cost;
                        }
                    }};
                }
                if at.0 != 0 {
                    walk!(at.0 - 1, at.1);
                }
                if at.1 != 0 {
                    walk!(at.0, at.1 - 1);
                }
                if at.0 != max_x {
                    walk!(at.0 + 1, at.1);
                }
                if at.1 != max_y {
                    walk!(at.0, at.1 + 1);
                }
            }
        }
        unreachable!()
    }
}

impl Runner for Day15 {
    fn parse(&mut self, file: &[u8], part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let mut grid_line: Vec<u8> = line.chars().map(|c| c as u8 - b'0').collect();
            if part == 2 {
                let len = grid_line.len();
                for a in 1..5 {
                    for i in 0..len {
                        let v = grid_line[i] + a;
                        if v > 9 {
                            grid_line.push(v - 9);
                        } else {
                            grid_line.push(v);
                        }
                    }
                }
            }
            self.grid.push(grid_line);
        }
        if part == 2 {
            let len = self.grid.len();
            for a in 1..5 {
                for b in 0..len {
                    let grid_line = self.grid[b]
                        .iter()
                        .map(|c| {
                            let v = c + a;
                            if v > 9 {
                                v - 9
                            } else {
                                v
                            }
                        })
                        .collect();
                    self.grid.push(grid_line);
                }
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

impl Day15 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_path().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_path().into())
    }
}
