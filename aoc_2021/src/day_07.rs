#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day07 {
    crabs: Vec<usize>,
    costs: Vec<usize>,
    max: usize,
}

impl Day07 {
    pub fn new() -> Self {
        Self {
            crabs: Vec::new(),
            costs: Vec::new(),
            max: 0,
        }
    }

    fn make_costs(&mut self, non_const: bool) {
        let mut next = 1;
        self.costs.push(0);
        for step in 1..=self.max {
            self.costs.push(next);
            next += if !non_const { 1 } else { step + 1 };
        }
    }

    fn calc_min_cost(&self) -> usize {
        let mut min = usize::MAX;
        for pos in 1..=self.max {
            let cost = self
                .crabs
                .iter()
                .map(|c| self.costs[(*c as isize - pos as isize).unsigned_abs()])
                .sum();
            min = min.min(cost);
        }

        min
    }
}

impl Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.crabs
            .extend(lines[0].split(',').map(|c| c.parse::<usize>().unwrap()));
        self.max = *self.crabs.iter().max().unwrap();
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day07 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.make_costs(false);
        Ok(self.calc_min_cost().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.make_costs(true);
        Ok(self.calc_min_cost().into())
    }
}
