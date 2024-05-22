#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Copy, Clone)]
enum Op {
    Const(isize),
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
}

pub struct Day21 {
    names: BTreeMap<String, usize>,
    monkeys: Vec<Op>,
    root: usize,
    humn: usize,
}

impl Day21 {
    pub fn new() -> Self {
        Self {
            names: BTreeMap::new(),
            monkeys: Vec::new(),
            root: usize::MAX,
            humn: usize::MAX,
        }
    }

    fn find_op(&mut self, name: &str) -> usize {
        if let Some(idx) = self.names.get(name) {
            *idx
        } else {
            self.names.insert(name.into(), self.monkeys.len());
            self.monkeys.push(Op::Const(isize::MIN));
            self.monkeys.len() - 1
        }
    }

    fn resolve(&self) -> isize {
        fn resolve_op(ops: &mut [Op], idx: usize) -> isize {
            let a = match ops[idx] {
                Op::Const(c) => return c,
                Op::Add(a, b) => {
                    let a = resolve_op(ops, a);
                    let b = resolve_op(ops, b);
                    a + b
                }
                Op::Sub(a, b) => {
                    let a = resolve_op(ops, a);
                    let b = resolve_op(ops, b);
                    a - b
                }
                Op::Mul(a, b) => {
                    let a = resolve_op(ops, a);
                    let b = resolve_op(ops, b);
                    a * b
                }
                Op::Div(a, b) => {
                    let a = resolve_op(ops, a);
                    let b = resolve_op(ops, b);
                    a / b
                }
            };
            ops[idx] = Op::Const(a);
            a
        }

        resolve_op(&mut self.monkeys.clone(), self.root)
    }
}

impl Runner for Day21 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for line in lines.iter() {
            let (name, op) = line.split_once(": ").unwrap();

            let name_idx = self.find_op(name);
            self.monkeys[name_idx] = if let Some((a, b)) = op.split_once(" + ") {
                Op::Add(self.find_op(a), self.find_op(b))
            } else if let Some((a, b)) = op.split_once(" - ") {
                Op::Sub(self.find_op(a), self.find_op(b))
            } else if let Some((a, b)) = op.split_once(" * ") {
                Op::Mul(self.find_op(a), self.find_op(b))
            } else if let Some((a, b)) = op.split_once(" / ") {
                Op::Div(self.find_op(a), self.find_op(b))
            } else {
                Op::Const(op.parse()?)
            };
        }
        self.root = *self.names.get("root").unwrap();
        self.humn = *self.names.get("humn").unwrap();

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.resolve().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let root = &mut self.monkeys[self.root];
        *root = match root {
            Op::Add(a, b) => Op::Sub(*a, *b),
            Op::Sub(a, b) => Op::Sub(*a, *b),
            Op::Mul(a, b) => Op::Sub(*a, *b),
            Op::Div(a, b) => Op::Sub(*a, *b),
            _ => unreachable!(),
        };

        let mut h = 0;
        self.monkeys[self.humn] = Op::Const(h);
        let root = self.resolve();
        let initial = root.is_negative();
        let mut scale = 1;
        while scale * 10 < root.abs() {
            scale *= 10;
        }

        loop {
            let cur_h = h + scale;
            self.monkeys[self.humn] = Op::Const(cur_h);
            let root = self.resolve();
            if root == 0 {
                return Ok(cur_h.into());
            }
            if (initial && root.is_positive()) || (!initial && root.is_negative()) {
                // Lower the scale and try again
                scale /= 10;
            } else {
                h += scale;
            }
        }
    }
}
