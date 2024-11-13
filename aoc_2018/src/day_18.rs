#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day18 {
    cur: usize,
    tiles: [Vec<Vec<char>>; 2],
    neighbor_cache: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Day18 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_counts(&self, x: usize, y: usize) -> (char, usize, usize, usize) {
        let mut open = 0;
        let mut tree = 0;
        let mut lumberyard = 0;

        for neighbor in self.neighbor_cache.get(&(x, y)).unwrap() {
            match self.tiles[self.cur][neighbor.1][neighbor.0] {
                '.' => open += 1,
                '|' => tree += 1,
                '#' => lumberyard += 1,
                _ => {}
            }
        }

        (self.tiles[self.cur][y][x], open, tree, lumberyard)
    }

    fn get_resource_counts(&self) -> (usize, usize) {
        let mut trees = 0;
        let mut lumberyards = 0;
        for y in 0..self.tiles[self.cur].len() {
            for x in 0..self.tiles[self.cur][y].len() {
                match self.tiles[self.cur][y][x] {
                    '|' => trees += 1,
                    '#' => lumberyards += 1,
                    _ => {}
                }
            }
        }

        (trees, lumberyards)
    }

    fn mutate(&mut self) {
        let next = 1 - self.cur;

        for y in 0..self.tiles[self.cur].len() {
            for x in 0..self.tiles[self.cur][y].len() {
                let (c, _open, tree, lumberyard) = self.get_counts(x, y);
                if c == '.' {
                    if tree >= 3 {
                        self.tiles[next][y][x] = '|';
                    } else {
                        self.tiles[next][y][x] = '.';
                    }
                } else if c == '|' {
                    if lumberyard >= 3 {
                        self.tiles[next][y][x] = '#';
                    } else {
                        self.tiles[next][y][x] = '|';
                    }
                } else if c == '#' {
                    if lumberyard >= 1 && tree >= 1 {
                        self.tiles[next][y][x] = '#';
                    } else {
                        self.tiles[next][y][x] = '.';
                    }
                }
            }
        }
        self.cur = next;
    }

    fn build_neighbors_map(&mut self) {
        for y in 0..self.tiles[0].len() {
            for x in 0..self.tiles[0][0].len() {
                let mut neighbors = Vec::new();
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let x = x as isize + dx;
                        let y = y as isize + dy;

                        if x < 0 || y < 0 {
                            continue;
                        }
                        let x = x as usize;
                        let y = y as usize;
                        if x >= self.tiles[0][0].len() || y >= self.tiles[0].len() {
                            continue;
                        }

                        neighbors.push((x, y));
                    }
                }

                self.neighbor_cache.insert((x, y), neighbors);
            }
        }
    }

    fn simulate(&mut self, minutes: usize) -> usize {
        let mut seen = Vec::new();

        seen.push(self.tiles[self.cur].clone());
        for min in 1..=minutes {
            self.mutate();
            for i in 0..seen.len() {
                if seen[i].eq(&self.tiles[self.cur]) {
                    let idx = ((minutes - min) % (min - i)) + i;
                    self.tiles[self.cur] = seen[idx].clone();
                    let (num_trees, num_lumberyards) = self.get_resource_counts();
                    return num_trees * num_lumberyards;
                }
            }
            seen.push(self.tiles[self.cur].clone());
        }

        let (num_trees, num_lumberyards) = self.get_resource_counts();
        num_trees * num_lumberyards
    }
}

impl Runner for Day18 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let line: Vec<char> = line.chars().collect();
            self.tiles[1].push(vec!['.'; line.len()]);
            self.tiles[0].push(line);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.build_neighbors_map();
        Ok(self.simulate(10).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.build_neighbors_map();
        Ok(self.simulate(1000000000).into())
    }
}
