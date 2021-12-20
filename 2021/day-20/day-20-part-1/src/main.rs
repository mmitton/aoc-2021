#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

#[derive(Debug, Clone)]
struct Cell {
    x: isize,
    y: isize,
}

struct Grid {
    cells: Vec<Cell>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    infinite_on: bool,
}

impl Grid {
    fn get_cell(&self, x: isize, y: isize) -> bool {
        if x < self.min_x || x > self.max_x || y < self.min_y || y > self.max_y {
            return self.infinite_on;
        }

        for cell in &self.cells {
            if cell.x == x && cell.y == y {
                return true;
            }
        }

        false
    }

    fn num_lit(&self) -> usize {
        self.cells.len()
    }

    fn enchance(&self, enhance_key: &Vec<char>) -> Grid {
        let mut grid = Grid {
            cells: Vec::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            infinite_on: if enhance_key[0] == '#' {
                !self.infinite_on
            } else {
                false
            },
        };

        for y in self.min_y - 1..=self.max_y + 1 {
            for x in self.min_x - 1..=self.max_x + 1 {
                let mut index = 0usize;
                for y1 in -1..=1 {
                    for x1 in -1..=1 {
                        index <<= 1;
                        if self.get_cell(x + x1, y + y1) {
                            index |= 1;
                        }
                    }
                }

                if enhance_key[index] == '#' {
                    if grid.cells.len() == 0 {
                        grid.min_x = x;
                        grid.max_x = x;
                        grid.min_y = y;
                        grid.max_y = y;
                    } else {
                        if x < grid.min_x {
                            grid.min_x = x
                        }
                        if x > grid.max_x {
                            grid.max_x = x
                        }
                        if y < grid.min_y {
                            grid.min_y = y
                        }
                        if y > grid.max_y {
                            grid.max_y = y
                        }
                    }
                    grid.cells.push(Cell { x: x, y: y });
                }
            }
        }

        grid
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in self.min_y - 10..=self.max_y + 10 {
            for x in self.min_x - 10..=self.max_x + 10 {
                if self.get_cell(x, y) {
                    write!(fmt, "#")?;
                } else {
                    write!(fmt, ".")?;
                }
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

fn load_input(filename: &str) -> Result<(Grid, Vec<char>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut grid = Grid {
        cells: Vec::new(),
        min_x: 0,
        max_x: 0,
        min_y: 0,
        max_y: 0,
        infinite_on: false,
    };
    let mut enhance_key: Vec<char> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if enhance_key.len() == 0 {
            enhance_key.extend_from_slice(&line.chars().collect::<Vec<char>>());
        } else {
            let chars: Vec<char> = line.chars().collect();
            for x in 0..chars.len() {
                if chars[x] == '#' {
                    grid.cells.push(Cell {
                        x: x as isize,
                        y: grid.max_y,
                    });
                }
            }

            grid.max_x = line.len() as isize - 1;
            grid.max_y += 1;
        }
    }
    grid.max_y -= 1;

    Ok((grid, enhance_key))
}

fn main() -> Result<(), Error> {
    let (mut grid, enhance_key) = load_input(INPUT_FILE)?;

    println!("Grid:\n{}", grid);

    for i in 0..2 {
        grid = grid.enchance(&enhance_key);
        println!("Enchance {}:\n{}", i, grid);
    }

    println!("Num Lit: {}", grid.num_lit());
    // 5423 too low

    Ok(())
}
