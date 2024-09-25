#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

macro_rules! tile_bit {
    ($x:expr, $y:expr) => {
        (1 << (($y) * 5) + ($x))
    };
}

pub struct Day24 {
    tiles: [u32; 256],
    neighbors: Vec<Vec<Vec<(u32, u8)>>>,
    z_min: u8,
    z_max: u8,
}

impl Day24 {
    const LEVEL_0: u8 = 128;
    pub fn new() -> Self {
        Self {
            tiles: [0; 256],
            neighbors: Vec::new(),
            z_min: Self::LEVEL_0,
            z_max: Self::LEVEL_0,
        }
    }

    fn neighbor_count(&self, x: u8, y: u8, z: u8) -> u8 {
        self.neighbors[y as usize][x as usize]
            .iter()
            .map(|(bit, dz)| {
                if self.tiles[z.wrapping_add(*dz) as usize] & bit != 0 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn tick(&mut self, recurse: bool) {
        let mut next_tiles = [0; 256];

        for z in self.z_min - 1..=self.z_max + 1 {
            for y in 0..5 {
                for x in 0..5 {
                    if recurse && y == 2 && x == 2 {
                        continue;
                    }
                    let neighbor_count = self.neighbor_count(x, y, z);
                    let bit = tile_bit!(x, y);
                    if self.tiles[z as usize] & bit != 0 {
                        if neighbor_count == 1 {
                            next_tiles[z as usize] |= bit;
                            self.z_min = self.z_min.min(z);
                            self.z_max = self.z_max.max(z);
                        }
                    } else if neighbor_count == 1 || neighbor_count == 2 {
                        next_tiles[z as usize] |= bit;
                        self.z_min = self.z_min.min(z);
                        self.z_max = self.z_max.max(z);
                    }
                }
            }
        }

        std::mem::swap(&mut self.tiles, &mut next_tiles);
    }
}

impl Runner for Day24 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => self.tiles[Self::LEVEL_0 as usize] |= tile_bit!(x, y),
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        for y in 0..5 {
            let mut row = Vec::new();
            for x in 0..5 {
                let mut cell = Vec::new();

                // Calc up/down tiles
                if y == 0 {
                    cell.push((tile_bit!(x, y + 1), 0));
                } else if y == 4 {
                    cell.push((tile_bit!(x, y - 1), 0));
                } else {
                    cell.push((tile_bit!(x, y - 1), 0));
                    cell.push((tile_bit!(x, y + 1), 0));
                }

                // Calc left/right tiles
                if x == 0 {
                    cell.push((tile_bit!(x + 1, y), 0));
                } else if x == 4 {
                    cell.push((tile_bit!(x - 1, y), 0));
                } else {
                    cell.push((tile_bit!(x - 1, y), 0));
                    cell.push((tile_bit!(x + 1, y), 0));
                }
                row.push(cell);
            }
            self.neighbors.push(row);
        }
        let mut seen = std::collections::HashSet::new();
        seen.insert(self.tiles[Self::LEVEL_0 as usize]);
        loop {
            self.tick(false);
            if !seen.insert(self.tiles[Self::LEVEL_0 as usize]) {
                break;
            }
        }
        Ok(self.tiles[Self::LEVEL_0 as usize].into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for y in 0..5 {
            let mut row = Vec::new();
            for x in 0..5 {
                let mut cell = Vec::new();

                // Calc up/down tiles
                if y == 0 {
                    cell.push((tile_bit!(2, 1), 255));
                    cell.push((tile_bit!(x, 1), 0));
                } else if y == 4 {
                    cell.push((tile_bit!(x, 3), 0));
                    cell.push((tile_bit!(2, 3), 255));
                } else if y == 1 && x == 2 {
                    cell.push((tile_bit!(0, 0), 1));
                    cell.push((tile_bit!(1, 0), 1));
                    cell.push((tile_bit!(2, 0), 1));
                    cell.push((tile_bit!(3, 0), 1));
                    cell.push((tile_bit!(4, 0), 1));
                    cell.push((tile_bit!(x, y - 1), 0));
                } else if y == 3 && x == 2 {
                    cell.push((tile_bit!(0, 4), 1));
                    cell.push((tile_bit!(1, 4), 1));
                    cell.push((tile_bit!(2, 4), 1));
                    cell.push((tile_bit!(3, 4), 1));
                    cell.push((tile_bit!(4, 4), 1));
                    cell.push((tile_bit!(x, y + 1), 0));
                } else {
                    cell.push((tile_bit!(x, y - 1), 0));
                    cell.push((tile_bit!(x, y + 1), 0));
                }

                // Calc left/right tiles
                if x == 0 {
                    cell.push((tile_bit!(1, 2), 255));
                    cell.push((tile_bit!(1, y), 0));
                } else if x == 4 {
                    cell.push((tile_bit!(3, y), 0));
                    cell.push((tile_bit!(3, 2), 255));
                } else if x == 1 && y == 2 {
                    cell.push((tile_bit!(0, 0), 1));
                    cell.push((tile_bit!(0, 1), 1));
                    cell.push((tile_bit!(0, 2), 1));
                    cell.push((tile_bit!(0, 3), 1));
                    cell.push((tile_bit!(0, 4), 1));
                    cell.push((tile_bit!(x - 1, y), 0));
                } else if x == 3 && y == 2 {
                    cell.push((tile_bit!(4, 0), 1));
                    cell.push((tile_bit!(4, 1), 1));
                    cell.push((tile_bit!(4, 2), 1));
                    cell.push((tile_bit!(4, 3), 1));
                    cell.push((tile_bit!(4, 4), 1));
                    cell.push((tile_bit!(x + 1, y), 0));
                } else {
                    cell.push((tile_bit!(x - 1, y), 0));
                    cell.push((tile_bit!(x + 1, y), 0));
                }

                row.push(cell);
            }
            self.neighbors.push(row);
        }

        for _ in 1..=200 {
            self.tick(true);
        }

        Ok(self
            .tiles
            .iter()
            .map(|level| level.count_ones())
            .sum::<u32>()
            .into())
    }
}
