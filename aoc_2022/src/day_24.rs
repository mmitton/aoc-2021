#[allow(unused_imports)]
use helper::{print, println, BitGridConst, Error, Lines, LinesOpt, Output, RunOutput, Runner};

const MIN_X: isize = 0;
const MIN_Y: isize = 0;
const WIDTH: usize = 128;
const HEIGHT: usize = 28;

struct Storm {
    at: (isize, isize),
    delta: (isize, isize),
}

pub struct Day24 {
    storms: Vec<Storm>,
    storm_grid: BitGridConst<MIN_X, MIN_Y, WIDTH, HEIGHT>,
    pos_grid: BitGridConst<MIN_X, MIN_Y, WIDTH, HEIGHT>,
    next_pos_grid: BitGridConst<MIN_X, MIN_Y, WIDTH, HEIGHT>,
    start: (isize, isize),
    end: (isize, isize),
    max: (isize, isize),
}

impl Day24 {
    pub fn new() -> Self {
        Self {
            storms: Vec::new(),
            storm_grid: BitGridConst::default(),
            pos_grid: BitGridConst::default(),
            next_pos_grid: BitGridConst::default(),
            start: (isize::MIN, isize::MIN),
            end: (isize::MIN, isize::MIN),
            max: (isize::MAX, isize::MIN),
        }
    }

    fn traverse(&mut self, from: (isize, isize), to: (isize, isize)) -> usize {
        self.pos_grid.clear();
        self.pos_grid.set_bit(from.0, from.1);
        for step in 0.. {
            self.next_pos_grid.clear();
            for y in 0..=self.max.1 {
                for x in 0..=self.max.0 {
                    if self.pos_grid.bit_is_set(x, y) && !self.storm_grid.bit_is_set(x, y) {
                        self.next_pos_grid.set_bit(x, y);
                        if y == 0 || y == self.max.1 {
                            // Inside a exit
                            if y == to.1 {
                                return step;
                            }
                            if y == 0 {
                                self.next_pos_grid.set_bit(x, y + 1);
                            } else {
                                self.next_pos_grid.set_bit(x, y - 1);
                            }
                        } else {
                            // Inside the area
                            if x > 1 {
                                self.next_pos_grid.set_bit(x - 1, y);
                            }
                            if y > 1 || (to.1 == 0 && x == to.0) {
                                self.next_pos_grid.set_bit(x, y - 1);
                            }
                            if x < self.max.0 - 1 {
                                self.next_pos_grid.set_bit(x + 1, y);
                            }
                            if y < self.max.1 - 1 || (to.1 == self.max.1 && x == to.0) {
                                self.next_pos_grid.set_bit(x, y + 1);
                            }
                        }
                    }
                }
            }
            std::mem::swap(&mut self.pos_grid, &mut self.next_pos_grid);
            // Move the storms
            self.storm_grid.clear();
            for storm in self.storms.iter_mut() {
                storm.at.0 += storm.delta.0;
                storm.at.1 += storm.delta.1;
                if storm.at.0 == 0 {
                    storm.at.0 = self.max.0 - 1;
                }
                if storm.at.0 == self.max.0 {
                    storm.at.0 = 1;
                }
                if storm.at.1 == 0 {
                    storm.at.1 = self.max.1 - 1;
                }
                if storm.at.1 == self.max.1 {
                    storm.at.1 = 1;
                }
                self.storm_grid.set_bit(storm.at.0, storm.at.1);
            }
        }
        0
    }
}

impl Runner for Day24 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let mut lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        use std::ops::DerefMut;

        self.start.1 = 0;
        self.end.1 = lines.len() as isize - 1;
        self.max.0 = lines[0].len() as isize - 1;
        self.max.1 = lines.len() as isize - 1;

        self.start.0 = lines
            .first()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap() as isize;
        self.end.0 = lines
            .deref_mut()
            .pop()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap() as isize;

        for (y, line) in lines.iter().enumerate().skip(1) {
            let y = y as isize;
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                match c {
                    '>' => self.storms.push(Storm {
                        at: (x, y),
                        delta: (1, 0),
                    }),
                    '<' => self.storms.push(Storm {
                        at: (x, y),
                        delta: (-1, 0),
                    }),
                    '^' => self.storms.push(Storm {
                        at: (x, y),
                        delta: (0, -1),
                    }),
                    'v' => self.storms.push(Storm {
                        at: (x, y),
                        delta: (0, 1),
                    }),
                    '.' | '#' => {}
                    _ => unreachable!(),
                }
            }
        }
        for storm in self.storms.iter_mut() {
            self.storm_grid.set_bit(storm.at.0, storm.at.1);
        }

        println!("{:?} {:?}", self.start, self.end);
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.traverse(self.start, self.end).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let a = self.traverse(self.start, self.end);
        let b = self.traverse(self.end, self.start);
        let c = self.traverse(self.start, self.end);
        Ok((a + b + c).into())
    }
}
