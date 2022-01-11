#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

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

    Ok(Tiles {
        cur: 0,
        tiles: [tiles, working],
    })
}

struct Tiles {
    cur: usize,
    tiles: [Vec<Vec<char>>; 2],
}

impl Tiles {
    fn get_counts(&self, x: usize, y: usize) -> (char, usize, usize, usize) {
        let mut open = 0;
        let mut tree = 0;
        let mut lumberyard = 0;

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

                match self.tiles[self.cur][y][x] {
                    '.' => open += 1,
                    '|' => tree += 1,
                    '#' => lumberyard += 1,
                    _ => {}
                }
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

    fn print(&self) {
        for y in 0..self.tiles[self.cur].len() {
            for x in 0..self.tiles[self.cur][y].len() {
                print!("{}", self.tiles[self.cur][y][x]);
            }
            println!();
        }
        println!();
    }
}

fn main() -> Result<(), Error> {
    let mut tiles = load_input(INPUT_FILE)?;

    println!("Initial state:");
    tiles.print();

    for min in 1..=10 {
        tiles.mutate();
        println!("After {} minutes:", min);
        tiles.print();
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
