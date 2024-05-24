#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Copy, Clone)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

pub struct Day08 {
    inst: Vec<(Op, isize)>,
}

impl Day08 {
    pub fn new() -> Self {
        Self { inst: Vec::new() }
    }

    fn run(&self) -> Result<isize, isize> {
        let mut pc = 0;
        let mut acc = 0;
        let mut seen = HashSet::default();
        loop {
            if pc == self.inst.len() {
                return Ok(acc);
            } else if pc > self.inst.len() {
                unreachable!();
            } else if !seen.insert(pc) {
                return Err(acc);
            }
            match self.inst[pc] {
                (Op::Nop, _) => pc += 1,
                (Op::Acc, num) => {
                    acc += num;
                    pc += 1;
                }
                (Op::Jmp, num) => pc = pc.wrapping_add(num as usize),
            }
        }
    }
}

impl Runner for Day08 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for line in lines.iter() {
            let (op, num) = line.split_once(' ').unwrap();
            let num: isize = num.parse()?;
            let op = match op {
                "nop" => Op::Nop,
                "acc" => Op::Acc,
                "jmp" => Op::Jmp,
                _ => unreachable!(),
            };

            self.inst.push((op, num));
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        match self.run() {
            Ok(_) => unreachable!(),
            Err(num) => Ok(num.into()),
        }
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for i in 0..self.inst.len() {
            let prev = match self.inst[i].0 {
                Op::Nop => {
                    self.inst[i].0 = Op::Jmp;
                    Op::Nop
                }
                Op::Jmp => {
                    self.inst[i].0 = Op::Nop;
                    Op::Jmp
                }
                Op::Acc => continue,
            };

            if let Ok(num) = self.run() {
                return Ok(num.into());
            }
            self.inst[i].0 = prev;
        }
        Err(Error::Unsolved)
    }
}
