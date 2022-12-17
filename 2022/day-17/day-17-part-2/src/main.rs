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

    const ROCKS: usize = 1000000000000;
    const CYCLE_COUNT: usize = 2;

    for line in lines {
        let jets: Vec<char> = line.trim().chars().collect();
        let mut heights: Vec<isize> = Vec::new();

        let mut game = Game::new(jets);
        'drops: for top in 0..ROCKS {
            game.drop(BLOCKS[top % BLOCKS.len()]);
            heights.push(game.max_y());

            if heights.len() > 10 && heights[top - 1] > heights[top - 2] {
                'cycle: for cycle in 10..(top / (CYCLE_COUNT + 1)) {
                    let delta = heights[top] - heights[top - cycle];
                    for i in 0..=cycle {
                        for c in 0..CYCLE_COUNT {
                            if heights[top - i] - heights[top - ((c + 1) * cycle) - i]
                                != (c as isize + 1) * delta
                            {
                                continue 'cycle;
                            }
                        }
                    }
                    println!("{top} {cycle}");
                    let cycle_diff = heights[top] - heights[top - cycle];
                    let rocks_left = ROCKS - top;
                    let cycles = rocks_left / cycle;
                    let left_over = ROCKS - top - (cycles * cycle);

                    let left_over_height =
                        heights[top - cycle + left_over - 1] - heights[top - cycle];

                    let ans = heights[top] + (cycles as isize * cycle_diff) + left_over_height;

                    println!("{cycle_diff} {rocks_left} {cycles} {left_over} {left_over_height}");
                    println!("ans: {ans}");
                    break 'drops;
                }
            }
        }
    }
}
