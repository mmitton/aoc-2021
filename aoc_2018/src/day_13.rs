#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Cart {
    y: usize,
    x: usize,
    dir: u8,
    next_turn: u8,
    crashed: bool,
}

#[derive(Default)]
pub struct Day13 {
    carts: Vec<Cart>,
    map: Vec<Vec<char>>,
}

impl Day13 {
    pub fn new() -> Self {
        Self::default()
    }

    fn move_carts(&mut self, first_crash: bool) -> Option<(usize, usize)> {
        self.carts.sort();
        for i in 0..self.carts.len() {
            if self.carts[i].crashed {
                continue;
            }

            match self.carts[i].dir {
                0 => self.carts[i].y -= 1,
                1 => self.carts[i].x += 1,
                2 => self.carts[i].y += 1,
                3 => self.carts[i].x -= 1,
                _ => unreachable!(),
            }

            // Check crash
            for j in 0..self.carts.len() {
                if i == j {
                    continue;
                }
                if self.carts[j].crashed {
                    continue;
                }
                if self.carts[i].x == self.carts[j].x && self.carts[i].y == self.carts[j].y {
                    if first_crash {
                        return Some((self.carts[i].x, self.carts[i].y));
                    }
                    self.carts[i].crashed = true;
                    self.carts[j].crashed = true;
                }
            }

            // Check for turn
            let c = self.map[self.carts[i].y][self.carts[i].x];
            match c {
                '/' => match self.carts[i].dir {
                    0 => self.carts[i].dir = 1,
                    1 => self.carts[i].dir = 0,
                    2 => self.carts[i].dir = 3,
                    3 => self.carts[i].dir = 2,
                    _ => unreachable!(),
                },
                '\\' => match self.carts[i].dir {
                    0 => self.carts[i].dir = 3,
                    1 => self.carts[i].dir = 2,
                    2 => self.carts[i].dir = 1,
                    3 => self.carts[i].dir = 0,
                    _ => unreachable!(),
                },
                '+' => {
                    match self.carts[i].next_turn {
                        0 => {
                            self.carts[i].dir = if self.carts[i].dir == 0 {
                                3
                            } else {
                                self.carts[i].dir - 1
                            }
                        }
                        1 => {}
                        2 => self.carts[i].dir = (self.carts[i].dir + 1) % 4,
                        _ => unreachable!(),
                    }
                    self.carts[i].next_turn = (self.carts[i].next_turn + 1) % 3;
                }
                _ => {}
            }
        }

        let mut still_alive = 0;
        let mut x = 0;
        let mut y = 0;
        for cart in self.carts.iter() {
            if !cart.crashed {
                still_alive += 1;
                x = cart.x;
                y = cart.y;
            }
        }

        if still_alive == 1 {
            Some((x, y))
        } else {
            None
        }
    }
}

impl Runner for Day13 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;

        for (y, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(match c {
                    '^' => {
                        self.carts.push(Cart {
                            y,
                            x,
                            dir: 0,
                            next_turn: 0,
                            crashed: false,
                        });
                        '|'
                    }
                    '>' => {
                        self.carts.push(Cart {
                            y,
                            x,
                            dir: 1,
                            next_turn: 0,
                            crashed: false,
                        });
                        '-'
                    }
                    'v' => {
                        self.carts.push(Cart {
                            y,
                            x,
                            dir: 2,
                            next_turn: 0,
                            crashed: false,
                        });
                        '|'
                    }
                    '<' => {
                        self.carts.push(Cart {
                            y,
                            x,
                            dir: 3,
                            next_turn: 0,
                            crashed: false,
                        });
                        '-'
                    }
                    '|' | '-' | '/' | '\\' | ' ' | '+' => c,
                    _ => unreachable!("wtf?  '{}'", c),
                });
            }
            self.map.push(row);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        loop {
            if let Some((x, y)) = self.move_carts(true) {
                return Ok(format!("{x},{y}").into());
            }
        }
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        loop {
            if let Some((x, y)) = self.move_carts(false) {
                return Ok(format!("{x},{y}").into());
            }
        }
    }
}
