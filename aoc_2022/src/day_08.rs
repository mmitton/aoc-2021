#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day08 {
    map: Vec<Vec<(u8, [u8; 4])>>,
    height: usize,
    width: usize,
}

impl Day08 {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            height: 0,
            width: 0,
        }
    }

    fn calc_seen(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let h = self.map[y][x].0;
                let mut counts = [0; 4];
                for x in (0..x).rev() {
                    counts[0] += 1;
                    if self.map[y][x].0 >= h {
                        break;
                    }
                }
                for x in x + 1..self.width {
                    counts[1] += 1;
                    if self.map[y][x].0 >= h {
                        break;
                    }
                }
                for y in (0..y).rev() {
                    counts[2] += 1;
                    if self.map[y][x].0 >= h {
                        break;
                    }
                }
                for y in y + 1..self.height {
                    counts[3] += 1;
                    if self.map[y][x].0 >= h {
                        break;
                    }
                }
                self.map[y][x].1 = counts;
            }
        }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        x == 0
            || y == 0
            || x == self.width - 1
            || y == self.height - 1
            || (self.map[y][x].1[0] as usize == x && self.map[y][0].0 < self.map[y][x].0)
            || (self.map[y][x].1[1] as usize == self.width - x - 1
                && self.map[y][self.width - 1].0 < self.map[y][x].0)
            || (self.map[y][x].1[2] as usize == y && self.map[0][x].0 < self.map[y][x].0)
            || (self.map[y][x].1[3] as usize == self.height - y - 1
                && self.map[self.height - 1][x].0 < self.map[y][x].0)
    }
}

impl Runner for Day08 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.map
                .push(line.chars().map(|c| (c as u8 - b'0', [0; 4])).collect());
        }
        self.height = self.map.len();
        self.width = self.map[0].len();
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.calc_seen();

        let mut ans = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_visible(x, y) {
                    ans += 1;
                }
            }
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.calc_seen();
        let mut ans = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let seen = &self.map[y][x].1;
                let score =
                    seen[0] as usize * seen[1] as usize * seen[2] as usize * seen[3] as usize;
                ans = ans.max(score);
            }
        }

        Ok(ans.into())
    }
}
