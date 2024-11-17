use crate::intcode::{IntCode, State};
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::cmp::Ordering;
use std::collections::BTreeSet;

pub struct Day13 {
    intcode: IntCode<isize>,
}

impl Day13 {
    pub fn new() -> Self {
        Self {
            intcode: IntCode::default(),
        }
    }
}

impl Runner for Day13 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        self.intcode.load(Lines::from_bufread(file, LinesOpt::RAW)?)
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day13 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut blocks = BTreeSet::new();
        let mut outputs = Vec::new();
        loop {
            match self.intcode.run() {
                State::HasOutput(v) => {
                    outputs.push(v);
                    if outputs.len() == 3 {
                        if outputs[2] != 2 {
                            blocks.remove(&(outputs[0], outputs[1]));
                        } else {
                            blocks.insert((outputs[0], outputs[1]));
                        }
                        outputs.clear();
                    }
                }
                State::Stopped => break,
                x => unreachable!("Unexpected state: {x:?}"),
            }
        }
        Ok(blocks.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut paddle = 0;
        let mut ball = 0;
        let mut score = 0;
        let mut outputs = Vec::new();
        self.intcode[0] = 2;
        loop {
            match self.intcode.run() {
                State::HasOutput(v) => {
                    outputs.push(v);
                    if outputs.len() == 3 {
                        match (outputs[0], outputs[1], outputs[2]) {
                            (-1, 0, s) => score = s,
                            (x, _, 4) => ball = x,
                            (x, _, 3) => paddle = x,
                            _ => {}
                        }
                        outputs.clear();
                    }
                }
                State::Stopped => break,
                State::WaitingForInput(..) => match paddle.cmp(&ball) {
                    Ordering::Less => self.intcode.input.push_back(1),
                    Ordering::Equal => self.intcode.input.push_front(0),
                    Ordering::Greater => self.intcode.input.push_front(-1),
                },
                x => unreachable!("Unexpected state: {x:?}"),
            }
        }
        Ok(score.into())
    }
}
