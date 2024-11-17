#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day10 {
    adapters: Vec<usize>,
}

impl Day10 {
    pub fn new() -> Self {
        Self {
            adapters: Vec::new(),
        }
    }
}

impl Runner for Day10 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.adapters
            .extend(lines.iter().map(|l| l.parse::<usize>().unwrap()));
        self.adapters.sort();
        self.adapters.insert(0, 0);
        self.adapters
            .push(self.adapters[self.adapters.len() - 1] + 3);
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

impl Day10 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut diff = [0; 4];

        for a in self.adapters.windows(2) {
            diff[a[1] - a[0]] += 1;
        }
        Ok((diff[1] * diff[3]).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut totals = vec![0usize; self.adapters.len()];
        totals[self.adapters.len() - 1] = 1;

        for i in (0..self.adapters.len()).rev() {
            for j in (0..i).rev() {
                if self.adapters[i] - self.adapters[j] <= 3 {
                    totals[j] += totals[i];
                }
            }
        }
        Ok(totals[0].into())
    }
}
