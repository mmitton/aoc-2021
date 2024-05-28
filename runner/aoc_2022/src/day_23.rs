#[allow(unused_imports)]
use helper::{
    print, println, BitGridConst, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput,
    Runner,
};

const MIN_X: isize = -128;
const MIN_Y: isize = -128;
const WIDTH: usize = 256;
const HEIGHT: usize = 256;

pub struct Day23 {
    elves: Vec<(isize, isize)>,
    cur: BitGridConst<MIN_X, MIN_Y, WIDTH, HEIGHT>,
    next: BitGridConst<MIN_X, MIN_Y, WIDTH, HEIGHT>,
    next_invalid: BitGridConst<MIN_X, MIN_Y, WIDTH, HEIGHT>,
    to_stay: Vec<(isize, isize)>,
    #[allow(clippy::type_complexity)]
    to_move: Vec<((isize, isize), (isize, isize), u16)>,
    step: usize,
}

impl Day23 {
    pub fn new() -> Self {
        Self {
            elves: Vec::new(),
            cur: BitGridConst::default(),
            next: BitGridConst::default(),
            next_invalid: BitGridConst::default(),

            to_stay: Vec::new(),
            to_move: Vec::new(),
            step: 0,
        }
    }

    fn build_next(&mut self) {
        self.to_stay.clear();
        self.to_move.clear();
        for elf in self.elves.iter() {
            let neighbors = self.cur.get_surround(elf.0, elf.1);
            if neighbors & 0b111_101_111 == 0 {
                self.to_stay.push(*elf);
            } else {
                self.to_move.push((*elf, *elf, neighbors));
            }
        }
        self.cur.clear();
        self.next.clear();
        self.next_invalid.clear();
    }

    fn step(&mut self) -> bool {
        const RULE: [(isize, isize, u16); 4] = [
            (0, -1, 0b111_000_000),
            (0, 1, 0b000_000_111),
            (-1, 0, 0b001_001_001),
            (1, 0, 0b100_100_100),
        ];

        self.build_next();

        if self.to_stay.len() == self.elves.len() {
            self.step += 1;
            return false;
        }

        for (elf, next, neighbors) in self.to_move.iter_mut() {
            for s in 0..4 {
                let rule = &RULE[(s + self.step) % 4];
                if rule.2 & *neighbors == 0 {
                    next.0 = elf.0 + rule.0;
                    next.1 = elf.1 + rule.1;
                    break;
                }
            }
            if self.next.bit_is_set(next.0, next.1) {
                self.next_invalid.set_bit(next.0, next.1);
            }
            self.next.set_bit(next.0, next.1);
        }

        self.elves.clear();
        for elf in self.to_stay.iter() {
            self.elves.push(*elf);
            self.cur.set_bit(elf.0, elf.1);
        }
        for (cur, next, _) in self.to_move.iter() {
            if !self.next_invalid.bit_is_set(next.0, next.1) {
                self.elves.push(*next);
                self.cur.set_bit(next.0, next.1);
            } else {
                self.elves.push(*cur);
                self.cur.set_bit(cur.0, cur.1);
            }
        }

        self.step += 1;
        true
    }

    fn _print(&self) {
        let mut min = (isize::MAX, isize::MAX);
        let mut max = (isize::MIN, isize::MIN);

        for elf in self.elves.iter() {
            min.0 = min.0.min(elf.0);
            min.1 = min.1.min(elf.1);
            max.0 = max.0.max(elf.0);
            max.1 = max.1.max(elf.1);
        }

        println!();
        println!("After {} steps ({min:?} {max:?})", self.step);
        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                if self.elves.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        for y in min.1 - 10..=max.1 + 10 {
            for x in min.0 - 10..=max.0 + 10 {
                if self.cur.bit_is_set(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl Runner for Day23 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    let x = x as isize;
                    let y = y as isize;
                    self.elves.push((x, y));
                    self.cur.set_bit(x, y);
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        for _ in 0..10 {
            self.step();
        }

        let mut min = (isize::MAX, isize::MAX);
        let mut max = (isize::MIN, isize::MIN);

        for elf in self.elves.iter() {
            min.0 = min.0.min(elf.0);
            min.1 = min.1.min(elf.1);
            max.0 = max.0.max(elf.0);
            max.1 = max.1.max(elf.1);
        }

        let w = (max.0 - min.0 + 1) as usize;
        let h = (max.1 - min.1 + 1) as usize;
        let tiles = w * h;

        Ok((tiles - self.elves.len()).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        loop {
            if !self.step() {
                return Ok(self.step.into());
            }
        }
    }
}
