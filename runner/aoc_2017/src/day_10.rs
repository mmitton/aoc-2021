#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::fmt::Write;

#[derive(Default)]
pub struct Day10 {
    numbers: Vec<u8>,
    lengths: Vec<usize>,
    pos: usize,
    skip: usize,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }

    fn reorder(&mut self, rounds: usize) {
        for _ in 0..rounds {
            for len in self.lengths.iter().copied() {
                for i in 0..len / 2 {
                    let idx1 = (self.pos + i) % self.numbers.len();
                    let idx2 = (self.pos + (len - i - 1)) % self.numbers.len();
                    let (a, b) = (self.numbers[idx1], self.numbers[idx2]);
                    self.numbers[idx2] = a;
                    self.numbers[idx1] = b;
                }
                self.pos = (self.pos + len + self.skip) % self.numbers.len();
                self.skip += 1;
            }
        }
    }
}

impl Runner for Day10 {
    fn parse(&mut self, file: &[u8], part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let max_number = if part1 {
            assert_eq!(lines.len(), 1);
            for n in lines[0].split(',') {
                self.lengths.push(n.parse()?);
            }

            if self.lengths.len() == 4 {
                4
            } else {
                255
            }
        } else {
            if lines.len() == 1 {
                for c in lines[0].chars() {
                    self.lengths.push(c as u8 as usize);
                }
            }
            self.lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
            255
        };
        self.numbers.extend(0..=max_number);

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.reorder(1);
        Ok(self
            .numbers
            .iter()
            .take(2)
            .map(|v| *v as usize)
            .product::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.reorder(64);
        let mut hash = String::with_capacity(32);
        for block in self.numbers.chunks(16) {
            let h = block.iter().fold(0, |h, v| h ^ v);
            write!(&mut hash, "{:02x}", h)?;
        }
        Ok(hash.into())
    }
}
