#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

const MIN_X: isize = -128;
const MIN_Y: isize = -128;
const WIDTH: usize = 256;
const HEIGHT: usize = 256;

pub struct Day23 {
    elves: Vec<(isize, isize)>,
    cur: BitGrid<MIN_X, MIN_Y, WIDTH, HEIGHT>,
    next: BitGrid<MIN_X, MIN_Y, WIDTH, HEIGHT>,
    next_invalid: BitGrid<MIN_X, MIN_Y, WIDTH, HEIGHT>,
    to_stay: Vec<(isize, isize)>,
    to_move: Vec<((isize, isize), (isize, isize), u16)>,
    step: usize,
}

type BitGridInner = usize;
struct BitGrid<const MIN_X: isize, const MIN_Y: isize, const WIDTH: usize, const HEIGHT: usize> {
    grid: Vec<BitGridInner>,
}

impl<const MIN_X: isize, const MIN_Y: isize, const WIDTH: usize, const HEIGHT: usize> Default
    for BitGrid<MIN_X, MIN_Y, WIDTH, HEIGHT>
{
    fn default() -> Self {
        assert_eq!(WIDTH % Self::BITS, 0);
        Self {
            grid: vec![0; (WIDTH / Self::BITS) * HEIGHT],
        }
    }
}

impl<const MIN_X: isize, const MIN_Y: isize, const WIDTH: usize, const HEIGHT: usize>
    BitGrid<MIN_X, MIN_Y, WIDTH, HEIGHT>
{
    const BITS: usize = BitGridInner::BITS as usize;

    fn index_bit(&self, x: isize, y: isize) -> (usize, BitGridInner) {
        let nx = (x - MIN_X) as usize;
        let ny = (y - MIN_Y) as usize;
        let pos = (ny * WIDTH) + nx;
        // println!("{x} {y} {nx} {ny} {pos} {} {:08b}", pos / 8, 1 << (pos % 8));
        (pos / Self::BITS, 1 << (pos % Self::BITS))
    }

    fn get_surround(&self, x: isize, y: isize) -> u16 {
        let (index, bit) = self.index_bit(x - 1, y - 1);
        let mut top = self.grid[index] >> bit.trailing_zeros();
        let mut middle = self.grid[index + (WIDTH / Self::BITS)] >> bit.trailing_zeros();
        let mut bottom = self.grid[index + (2 * WIDTH / Self::BITS)] >> bit.trailing_zeros();

        let extra_bits = Self::BITS - bit.trailing_zeros() as usize;
        if extra_bits < 3 {
            let extra_top = self.grid[index + 1] << extra_bits;
            let extra_middle = self.grid[index + 1 + (WIDTH / Self::BITS)] << extra_bits;

            let extra_bottom = self.grid[index + 1 + (2 * WIDTH / Self::BITS)] << extra_bits;
            top |= extra_top;
            middle |= extra_middle;
            bottom |= extra_bottom;
        }

        (((top & 0b111) << 6) | ((middle & 0b111) << 3) | (bottom & 0b111)) as u16
    }

    fn set_bit(&mut self, x: isize, y: isize) {
        let (index, bit) = self.index_bit(x, y);
        self.grid[index] |= bit;
    }

    fn _clear_bit(&mut self, x: isize, y: isize) {
        let (index, bit) = self.index_bit(x, y);
        self.grid[index] &= !bit;
    }

    fn bit_is_set(&self, x: isize, y: isize) -> bool {
        let (index, bit) = self.index_bit(x, y);
        self.grid[index] & bit != 0
    }

    fn clear(&mut self) {
        self.grid.iter_mut().for_each(|v| *v = 0);
    }
}

impl Day23 {
    pub fn new() -> Self {
        Self {
            elves: Vec::new(),
            cur: BitGrid::default(),
            next: BitGrid::default(),
            next_invalid: BitGrid::default(),

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
