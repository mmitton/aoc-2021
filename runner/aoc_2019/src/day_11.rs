use crate::intcode::{IntCode, State};
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{BTreeMap, BTreeSet};

pub struct Day11 {
    intcode: IntCode<i128>,
    painted: BTreeMap<(isize, isize), bool>,
}

impl Day11 {
    pub fn new() -> Self {
        Self {
            intcode: IntCode::default(),
            painted: BTreeMap::new(),
        }
    }

    fn run(&mut self) {
        let mut pos = (0, 0);
        let mut dir = 0;
        let deltas = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut outputs = Vec::new();
        loop {
            match self.intcode.run() {
                State::WaitingForInput(..) => {
                    let is_white = *self.painted.get(&pos).unwrap_or(&false);
                    self.intcode.input.push_back(if is_white { 1 } else { 0 });
                }
                State::HasOutput(v) => {
                    outputs.push(v);
                    if outputs.len() == 2 {
                        self.painted.insert(pos, outputs[0] == 1);
                        dir = (dir as isize + if outputs[1] == 0 { -1 } else { 1 }).rem_euclid(4)
                            as usize;
                        pos.0 += deltas[dir].0;
                        pos.1 += deltas[dir].1;
                        outputs.clear();
                    }
                }
                State::Stopped => break,
                x => unreachable!("Unexpected state: {x:?}"),
            }
        }
    }

    fn output(&self) -> String {
        let positions: BTreeSet<(isize, isize)> = self
            .painted
            .iter()
            .filter_map(|(&pos, &is_white)| if is_white { Some(pos) } else { None })
            .collect();

        let mut min = (isize::MAX, isize::MAX);
        let mut max = (isize::MIN, isize::MIN);
        for pos in positions.iter() {
            min.0 = min.0.min(pos.0);
            min.1 = min.1.min(pos.1);
            max.0 = max.0.max(pos.0);
            max.1 = max.1.max(pos.1);
        }

        let mut ret = String::with_capacity(((max.0 - min.0 + 2) * (max.1 - min.1 + 1)) as usize);
        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                ret.push(if positions.contains(&(x, y)) {
                    '#'
                } else {
                    ' '
                });
            }
            ret.push('\n');
        }
        ret
    }
}

impl Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        self.intcode.load(Lines::from_bufread(file, LinesOpt::RAW)?)
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.run();
        Ok(self.painted.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.painted.insert((0, 0), true);
        self.run();
        Ok(self.output().into())
    }
}
