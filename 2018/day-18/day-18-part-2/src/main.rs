#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<Tiles, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut tiles = Vec::new();
    let mut working = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        let line: Vec<char> = line.chars().collect();
        working.push(vec!['.'; line.len()]);
        tiles.push(line);
    }

    let mut neighbor_cache = BTreeMap::new();
    for y in 0..tiles.len() {
        for x in 0..tiles[0].len() {
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
                    if x >= tiles[0].len() || y >= tiles.len() {
                        continue;
                    }

                    neighbors.push((x, y));
                }
            }

            neighbor_cache.insert((x, y), neighbors);
        }
    }

    Ok(Tiles {
        cur: 0,
        tiles: [tiles, working],
        neighbor_cache: neighbor_cache,
    })
}

struct Tiles {
    cur: usize,
    tiles: [Vec<Vec<char>>; 2],
    neighbor_cache: BTreeMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Tiles {
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
}

fn main() -> Result<(), Error> {
    let mut tiles = load_input(INPUT_FILE)?;
    let mut seen = Vec::new();
    const MINUTES: usize = 1000000000;

    seen.push(tiles.tiles[tiles.cur].clone());
    for min in 1..=MINUTES {
        tiles.mutate();
        for i in 0..seen.len() {
            if seen[i].eq(&tiles.tiles[tiles.cur]) {
                println!("Found loop from {} to {}", i, min);
                let idx = ((MINUTES - min) % (min - i)) + i;
                tiles.tiles[tiles.cur] = seen[idx].clone();
                let (num_trees, num_lumberyards) = tiles.get_resource_counts();
                println!(
                    "{} * {} = {}",
                    num_trees,
                    num_lumberyards,
                    num_trees * num_lumberyards
                );
                return Ok(());
            }
        }
        seen.push(tiles.tiles[tiles.cur].clone());
    }

    let (num_trees, num_lumberyards) = tiles.get_resource_counts();
    println!(
        "{} * {} = {}",
        num_trees,
        num_lumberyards,
        num_trees * num_lumberyards
    );

    Ok(())
}
