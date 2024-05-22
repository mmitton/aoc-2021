#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

const BLOCKS: &[&[u8]] = &[
    &[0b0011110],
    &[0b0001000, 0b0011100, 0b0001000],
    &[0b0011100, 0b0000100, 0b0000100],
    &[0b0010000, 0b0010000, 0b0010000, 0b0010000],
    &[0b0011000, 0b0011000],
];

enum Dir {
    Left,
    Right,
}

pub struct Day17 {
    stack: Vec<u8>,
    jets: Vec<Dir>,
    tick: usize,
}

impl Day17 {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(8192),
            jets: Vec::new(),
            tick: 0,
        }
    }

    fn _print(&self, stack: &[u8]) {
        for v in stack.iter().rev() {
            print!("|");
            for shift in (0..7).rev() {
                print!("{}", if v >> shift & 1 == 1 { '#' } else { '.' });
            }
            println!("|");
        }
        println!();
    }

    fn gust(&mut self, bottom: usize, block: &mut [u8]) {
        match self.jets[self.tick % self.jets.len()] {
            Dir::Left => {
                if self.stack[bottom..bottom + block.len()]
                    .iter()
                    .zip(block.iter())
                    .all(|(s, v)| v & 0b01000000 == 0 && v << 1 & s == 0)
                {
                    block.iter_mut().for_each(|v| *v <<= 1);
                }
            }
            Dir::Right => {
                if self.stack[bottom..bottom + block.len()]
                    .iter()
                    .zip(block.iter())
                    .all(|(s, v)| v & 0b00000001 == 0 && v >> 1 & s == 0)
                {
                    block.iter_mut().for_each(|v| *v >>= 1);
                }
            }
        }
        self.tick += 1;

        // println!("After tick {}: bottom {}...", self.tick, bottom);
        // self._print(block);
    }

    fn drop_block(&mut self, block: &[u8]) {
        let mut block: Vec<u8> = block.into();
        let mut bottom = self.stack.len() + 3;
        self.stack.resize(self.stack.len() + 3 + block.len(), 0);

        while bottom != 0 {
            // Blast with jet
            self.gust(bottom, &mut block);

            // Drop block
            if !self.stack[bottom - 1..bottom + block.len()]
                .iter()
                .zip(block.iter())
                .all(|(s, v)| v & s == 0)
            {
                // Cannot drop
                break;
            }
            bottom -= 1;
        }

        if bottom == 0 {
            // Blast it one last time
            self.gust(bottom, &mut block);
        }

        // println!("Block landed at {bottom}");
        self.stack[bottom..bottom + block.len()]
            .iter_mut()
            .zip(block.iter())
            .for_each(|(s, v)| {
                *s |= v;
            });

        for top in (0..self.stack.len()).rev() {
            if self.stack[top] != 0 {
                self.stack.truncate(top + 1);
                break;
            }
        }
    }

    fn simulate(&mut self, total_drops: usize) -> usize {
        let mut heights: Vec<usize> = Vec::new();
        for drop in 0..total_drops {
            self.drop_block(BLOCKS[drop % BLOCKS.len()]);
            heights.push(self.stack.len());

            if drop >= 4096 {
                const CYCLE_COUNT: usize = 2;
                'cycle: for cycle in 256..(drop / (CYCLE_COUNT + 1)) {
                    let delta = heights[drop] - heights[drop - cycle];
                    for i in 0..=cycle {
                        for c in 0..CYCLE_COUNT {
                            if heights[drop - i] - heights[drop - ((c + 1) * cycle) - i]
                                != (c + 1) * delta
                            {
                                continue 'cycle;
                            }
                        }
                    }
                    println!("{drop} {cycle}");
                    let cycle_diff = heights[drop] - heights[drop - cycle];
                    let rocks_left = total_drops - drop;
                    let cycles = rocks_left / cycle;
                    let left_over = total_drops - drop - (cycles * cycle);

                    let left_over_height =
                        heights[drop - cycle + left_over - 1] - heights[drop - cycle];

                    let ans = heights[drop] + (cycles * cycle_diff) + left_over_height;

                    println!("{cycle_diff} {rocks_left} {cycles} {left_over} {left_over_height}");
                    println!("ans: {ans}");
                    return ans;
                }
            }

            // self._print(&self.stack);
        }

        self.stack.len()
    }
}

impl Runner for Day17 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.jets.extend(lines[0].chars().map(|c| match c {
            '>' => Dir::Right,
            '<' => Dir::Left,
            _ => unreachable!(),
        }));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        // Ok(self.simulate(4).into())
        Ok(self.simulate(2022).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.simulate(1000000000000).into())
        // Ok(0.into())
    }
}
