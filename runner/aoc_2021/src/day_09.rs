#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day09 {
    map: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Day09 {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            height: 0,
            width: 0,
        }
    }

    fn basins(&self) -> Vec<(usize, usize)> {
        let mut basins = Vec::new();
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let c = self.map[y][x];
                if c < self.map[y - 1][x]
                    && c < self.map[y + 1][x]
                    && c < self.map[y][x - 1]
                    && c < self.map[y][x + 1]
                {
                    basins.push((x, y));
                }
            }
        }
        basins
    }

    fn basin_size(&self, x: usize, y: usize) -> usize {
        let mut seen: HashSet<(usize, usize)> = HashSet::default();
        seen.insert((x, y));
        let mut work = vec![(x, y)];
        while let Some((x, y)) = work.pop() {
            macro_rules! add {
                ($x:expr, $y:expr) => {
                    if self.map[$y][$x] != 9 && seen.insert(($x, $y)) {
                        work.push(($x, $y));
                    }
                };
            }
            add!(x, y - 1);
            add!(x, y + 1);
            add!(x - 1, y);
            add!(x + 1, y);
        }
        seen.len()
    }
}

impl Runner for Day09 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        self.height = lines.len() + 2;
        self.width = lines[0].len() + 2;
        self.map
            .extend((0..self.height).map(|_| vec![9; self.width]));
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                self.map[y + 1][x + 1] = c as u8 - b'0';
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .basins()
            .iter()
            .fold(0, |ans, (x, y)| ans + 1 + self.map[*y][*x] as usize)
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut basins = self
            .basins()
            .iter()
            .map(|(x, y)| self.basin_size(*x, *y))
            .collect::<Vec<usize>>();
        basins.sort();
        Ok(basins.iter().rev().take(3).product::<usize>().into())
    }
}
