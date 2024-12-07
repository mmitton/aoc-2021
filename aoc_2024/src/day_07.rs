#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};
use std::str::FromStr;

struct Equation {
    test_value: usize,
    operands: Vec<usize>,
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test_value, operands_str) = s.split_once(':').ok_or(Error::InvalidInput(s.into()))?;
        let test_value = test_value.parse()?;
        let mut operands: Vec<usize> = Vec::new();

        for operand in operands_str.split_whitespace() {
            operands.push(operand.parse()?);
        }

        Ok(Self {
            test_value,
            operands,
        })
    }
}

impl Equation {
    fn reverse_solve(&self, has_combine: bool) -> bool {
        fn reverse_solve(target: usize, operands: &[usize], has_combine: bool) -> bool {
            if operands.len() == 1 {
                operands[0] == target
            } else {
                let last_idx = operands.len() - 1;
                let last = operands[last_idx];
                let operands = &operands[0..last_idx];
                if target > last && reverse_solve(target - last, operands, has_combine) {
                    return true;
                }
                if target % last == 0 && reverse_solve(target / last, operands, has_combine) {
                    return true;
                }
                if has_combine {
                    let mag = 10usize.pow(1 + last.ilog10());
                    if target % mag == last && reverse_solve(target / mag, operands, has_combine) {
                        return true;
                    }
                }
                false
            }
        }

        reverse_solve(self.test_value, &self.operands, has_combine)
    }
}

#[derive(Default)]
pub struct Day07 {
    equations: Vec<Equation>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .equations
            .iter()
            .filter_map(|e| {
                if e.reverse_solve(false) {
                    Some(e.test_value)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .equations
            .iter()
            .filter_map(|e| {
                if e.reverse_solve(true) {
                    Some(e.test_value)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.equations.push(line.parse()?);
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
