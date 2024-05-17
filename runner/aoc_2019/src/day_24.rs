#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::HashSet;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Tile {
    x: i8,
    y: i8,
    z: i8,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Tile {
    fn neighbor_count(
        &self,
        tiles: &HashSet<Tile>,
        recurse: bool,
        neighbors: &[Vec<Vec<Tile>>],
    ) -> u8 {
        if !recurse {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|(dx, dy)| {
                    if tiles.contains(&Tile {
                        x: self.x + dx,
                        y: self.y + dy,
                        z: self.z,
                    }) {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        } else {
            neighbors[self.y as usize][self.x as usize]
                .iter()
                .map(|neighbor| {
                    if tiles.contains(&Tile {
                        x: neighbor.x,
                        y: neighbor.y,
                        z: self.z + neighbor.z,
                    }) {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

pub struct Day24 {
    tiles: HashSet<Tile>,
    neighbors: Vec<Vec<Vec<Tile>>>,
    z_min: i8,
    z_max: i8,
}

impl Day24 {
    pub fn new() -> Self {
        Self {
            tiles: HashSet::new(),
            neighbors: Vec::new(),
            z_min: 0,
            z_max: 0,
        }
    }

    fn _print(&self, recurse: bool) {
        for z in self.z_min..=self.z_max {
            println!();
            println!("Depth {z}");
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 && recurse {
                        assert!(!self.tiles.contains(&Tile { x, y, z }));
                        print!("?");
                    } else if self.tiles.contains(&Tile { x, y, z }) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }

    fn tick(&mut self, recurse: bool) {
        let mut next_tiles = HashSet::new();

        for z in self.z_min - 1..=self.z_max + 1 {
            for y in 0..5 {
                for x in 0..5 {
                    if recurse && y == 2 && x == 2 {
                        continue;
                    }
                    let tile = Tile { x, y, z };
                    let neighbor_count = tile.neighbor_count(&self.tiles, recurse, &self.neighbors);
                    // if z == 1 && x == 4 && self.neighbors.len() > 0 {
                    //     println!(
                    //         "Tile {tile:?} is currently alive? {}  Has {} neighbors  {:?}",
                    //         self.tiles.contains(&tile),
                    //         neighbor_count,
                    //         self.neighbors[y as usize][x as usize]
                    //     );
                    // }
                    if self.tiles.contains(&tile) {
                        if neighbor_count == 1 {
                            next_tiles.insert(tile);
                            self.z_min = self.z_min.min(z);
                            self.z_max = self.z_max.max(z);
                        }
                    } else if neighbor_count == 1 || neighbor_count == 2 {
                        next_tiles.insert(tile);
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
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => {
                        let _ = self.tiles.insert(Tile {
                            x: x as i8,
                            y: y as i8,
                            z: 0,
                        });
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        fn tiles_to_u32(tiles: &HashSet<Tile>) -> u32 {
            tiles
                .iter()
                .fold(0, |acc, tile| acc | (1 << ((tile.y * 5) + tile.x)))
        }

        let mut seen = HashSet::new();
        seen.insert(tiles_to_u32(&self.tiles));
        loop {
            self.tick(false);
            if !seen.insert(tiles_to_u32(&self.tiles)) {
                break;
            }
        }
        Ok(tiles_to_u32(&self.tiles).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for y in 0..5 {
            let mut row = Vec::new();
            for x in 0..5 {
                let mut cell = Vec::new();

                // Calc up/down tiles
                if y == 0 {
                    cell.push(Tile { x: 2, y: 1, z: -1 });
                    cell.push(Tile { x, y: 1, z: 0 });
                } else if y == 4 {
                    cell.push(Tile { x, y: 3, z: 0 });
                    cell.push(Tile { x: 2, y: 3, z: -1 });
                } else if y == 1 && x == 2 {
                    cell.push(Tile { x: 0, y: 0, z: 1 });
                    cell.push(Tile { x: 1, y: 0, z: 1 });
                    cell.push(Tile { x: 2, y: 0, z: 1 });
                    cell.push(Tile { x: 3, y: 0, z: 1 });
                    cell.push(Tile { x: 4, y: 0, z: 1 });
                    cell.push(Tile { x, y: y - 1, z: 0 });
                } else if y == 3 && x == 2 {
                    cell.push(Tile { x: 0, y: 4, z: 1 });
                    cell.push(Tile { x: 1, y: 4, z: 1 });
                    cell.push(Tile { x: 2, y: 4, z: 1 });
                    cell.push(Tile { x: 3, y: 4, z: 1 });
                    cell.push(Tile { x: 4, y: 4, z: 1 });
                    cell.push(Tile { x, y: y + 1, z: 0 });
                } else {
                    cell.push(Tile { x, y: y - 1, z: 0 });
                    cell.push(Tile { x, y: y + 1, z: 0 });
                }

                // Calc left/right tiles
                if x == 0 {
                    cell.push(Tile { x: 1, y: 2, z: -1 });
                    cell.push(Tile { x: 1, y, z: 0 });
                } else if x == 4 {
                    cell.push(Tile { x: 3, y, z: 0 });
                    cell.push(Tile { x: 3, y: 2, z: -1 });
                } else if x == 1 && y == 2 {
                    cell.push(Tile { x: 0, y: 0, z: 1 });
                    cell.push(Tile { x: 0, y: 1, z: 1 });
                    cell.push(Tile { x: 0, y: 2, z: 1 });
                    cell.push(Tile { x: 0, y: 3, z: 1 });
                    cell.push(Tile { x: 0, y: 4, z: 1 });
                    cell.push(Tile { x: x - 1, y, z: 0 });
                } else if x == 3 && y == 2 {
                    cell.push(Tile { x: 4, y: 0, z: 1 });
                    cell.push(Tile { x: 4, y: 1, z: 1 });
                    cell.push(Tile { x: 4, y: 2, z: 1 });
                    cell.push(Tile { x: 4, y: 3, z: 1 });
                    cell.push(Tile { x: 4, y: 4, z: 1 });
                    cell.push(Tile { x: x + 1, y, z: 0 });
                } else {
                    cell.push(Tile { x: x - 1, y, z: 0 });
                    cell.push(Tile { x: x + 1, y, z: 0 });
                }

                row.push(cell);
            }
            self.neighbors.push(row);
        }

        for _ in 1..=200 {
            self.tick(true);
        }

        Ok(self.tiles.len().into())
    }
}
