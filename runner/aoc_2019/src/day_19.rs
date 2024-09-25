use crate::intcode::{IntCode, State};
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::ops::Range;

pub struct Day19 {
    intcode: IntCode<isize>,
    rows: Vec<Range<isize>>,
}

impl Day19 {
    pub fn new() -> Self {
        Self {
            intcode: IntCode::default(),
            rows: Vec::new(),
        }
    }

    fn scan_sector(&mut self, x: isize, y: isize) -> bool {
        let mut intcode = self.intcode.clone();
        intcode.input.push_back(x);
        intcode.input.push_back(y);
        match intcode.run() {
            State::HasOutput(v) => v == 1,
            x => unreachable!("Unexpected state: {x:?}"),
        }
    }

    fn run_scan(&mut self, is_done: impl Fn(&[Range<isize>]) -> bool) {
        let mut last_left = 0;
        let mut left = 0;
        let mut right = 0;
        const MAX_DIFF: isize = 10;
        for y in 0.. {
            while !self.scan_sector(left, y) && left - last_left < MAX_DIFF {
                left += 1;
            }
            if left - last_left == MAX_DIFF {
                left = last_left;
                self.rows.push(0..0);
                continue;
            }
            if left >= right {
                right = left + 1;
            }
            while self.scan_sector(right, y) {
                right += 1;
            }
            self.rows.push(left..right);
            if is_done(&self.rows) {
                break;
            }
            last_left = left;
        }
    }
}

impl Runner for Day19 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        self.intcode.load(Lines::from_bufread(file, LinesOpt::RAW)?)
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        const MAX: isize = 50;
        self.run_scan(|rows| rows.len() == MAX as usize);
        Ok(self
            .rows
            .iter()
            .map(|r| r.end.min(MAX - 1) - r.start.min(MAX - 1))
            .sum::<isize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        const SQUARE: isize = 100;
        self.run_scan(|rows| {
            if rows.len() < SQUARE as usize {
                false
            } else {
                let last_row = &rows[rows.len() - 1];
                let top_row = &rows[rows.len() - SQUARE as usize];
                last_row.end - last_row.start >= SQUARE && top_row.end - last_row.start >= SQUARE
            }
        });
        let last_row = &self.rows[self.rows.len() - 1];
        let x = last_row.start;
        let y = self.rows.len() as isize - SQUARE;
        Ok((x * 10_000 + y).into())
    }
}
