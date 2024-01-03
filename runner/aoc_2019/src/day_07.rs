use crate::intcode_multi::IntCodeMulti;
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day07 {
    amps: IntCodeMulti<i32>,
}

impl Day07 {
    pub fn new() -> Self {
        Self {
            amps: IntCodeMulti::default(),
        }
    }

    fn generate<F>(n: usize, a: &mut Vec<i32>, f: &mut F)
    where
        F: FnMut(&[i32]),
    {
        if n == 1 {
            f(a);
        } else {
            for i in 0..n - 1 {
                Self::generate(n - 1, a, f);

                if n % 2 == 0 {
                    a.swap(i, n - 1);
                } else {
                    a.swap(0, n - 1);
                }
            }
            Self::generate(n - 1, a, f);
        }
    }

    pub fn get_best(&self, low: i32) -> i32 {
        let mut numbers = (low..low + 5).collect();
        let mut best = 0;
        Self::generate(5, &mut numbers, &mut |vals| {
            let mut amps = self.amps.clone();
            for (i, v) in vals.iter().enumerate() {
                amps[i].input.push_front(*v);
            }
            amps[0].input.push_back(0);
            amps.run();
            best = best.max(amps.output.unwrap());
        });
        best
    }
}

impl Runner for Day07 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        self.amps.load(Lines::from_path(path, LinesOpt::RAW)?, 5)
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.get_best(0).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.amps.outputs[4].push(0);
        Ok(self.get_best(5).into())
    }
}
