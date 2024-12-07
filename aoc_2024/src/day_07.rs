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

#[derive(Copy, Clone)]
#[repr(transparent)]
struct Operator(u8);

impl Operator {
    fn next(&mut self, has_combine: bool) -> bool {
        self.0 = (self.0 + 1) % if has_combine { 3 } else { 2 };
        self.0 == 0
    }

    fn perform(&self, lhs: usize, rhs: usize) -> usize {
        match self.0 {
            0 => lhs + rhs,
            1 => lhs * rhs,
            2 => lhs * 10usize.pow(rhs.ilog10() + 1) + rhs,
            _ => 0,
        }
    }
}

struct Operators {
    operators: Vec<Operator>,
    has_combine: bool,
}

impl Operators {
    fn new(operands: usize, has_combine: bool) -> Self {
        Self {
            operators: vec![Operator(0); operands - 1],
            has_combine,
        }
    }

    fn next(&mut self) -> bool {
        for operator in self.operators.iter_mut() {
            if !operator.next(self.has_combine) {
                return true;
            }
        }
        false
    }

    fn solve(&self, operands: &[usize]) -> usize {
        let mut total = operands[0];
        for (operand, operator) in operands[1..].iter().zip(self.operators.iter()) {
            total = operator.perform(total, *operand);
        }

        total
    }
}

impl Equation {
    fn solve(&self, has_combine: bool) -> bool {
        let mut operators = Operators::new(self.operands.len(), has_combine);
        loop {
            if operators.solve(&self.operands) == self.test_value {
                return true;
            }
            if !operators.next() {
                break;
            }
        }
        false
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
                if e.solve(false) {
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
                if e.solve(true) {
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
