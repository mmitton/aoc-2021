const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Game {
    jets: Vec<char>,
    tick: usize,
    grid: BTreeSet<(isize, isize)>,
    piece: Vec<(isize, isize)>,
}

impl Game {
    fn new(jets: Vec<char>) -> Self {
        Self {
            jets,
            tick: 0,
            grid: BTreeSet::new(),
            piece: Vec::new(),
        }
    }

    fn max_y(&self) -> isize {
        let mut max_y = 0;
        for (_, y) in &self.grid {
            if *y > max_y {
                max_y = *y;
            }
        }
        for (_, y) in &self.piece {
            if *y > max_y {
                max_y = *y;
            }
        }

        max_y
    }

    fn drop(&mut self, piece: &[(isize, isize)]) {
        self.piece.clear();
        let grid_max_y = self.max_y();

        for cell in piece {
            let cell = (cell.0 + 3, cell.1 + grid_max_y + 4);
            self.piece.push(cell);
        }

        loop {
            let jet = self.jets[self.tick % self.jets.len()];
            self.tick += 1;

            // Check if we can push side to side
            let dx = if jet == '<' { -1 } else { 1 };
            let mut jet_ok = true;
            for cell in &self.piece {
                let cell = (cell.0 + dx, cell.1);
                if self.grid.contains(&cell) || cell.0 == 0 || cell.0 == 8 {
                    jet_ok = false;
                    break;
                }
            }
            if jet_ok {
                for cell in &mut self.piece {
                    cell.0 += dx;
                }
            }

            // Check if we can drop
            for cell_idx in 0..self.piece.len() {
                let cell = (self.piece[cell_idx].0, self.piece[cell_idx].1 - 1);
                if self.grid.contains(&cell) || cell.1 == 0 {
                    // Not ok to drop!  Move current piece in to the grid and return
                    for cell in &self.piece {
                        self.grid.insert(*cell);
                    }
                    return;
                }
            }

            for cell in &mut self.piece {
                cell.1 -= 1;
            }
        }
    }

    fn print(&self) {
        let grid_max_y = self.max_y();

        println!("{}", grid_max_y);
        for y in (1..=grid_max_y).rev() {
            print!("|");
            for x in 1..=7 {
                if self.grid.contains(&(x, y)) {
                    print!("#");
                } else if self.piece.contains(&(x, y)) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("+-------+");
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    static BLOCKS: &[&[(isize, isize)]] = &[
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        &[(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)],
        &[(0, 0), (0, 1), (0, 2), (0, 3)],
        &[(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    for line in lines {
        let jets: Vec<char> = line.trim().chars().collect();

        let mut game = Game::new(jets);
        for i in 0..2022 {
            game.drop(BLOCKS[i % BLOCKS.len()]);
        }

        println!("ans: {}", game.max_y());
    }
}
