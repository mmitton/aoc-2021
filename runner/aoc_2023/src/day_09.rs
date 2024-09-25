#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day09 {
    values: Vec<Vec<isize>>,
}

impl Day09 {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl Runner for Day09 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
            self.values.push(
                line.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            );
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;

        for values in self.values.iter() {
            ans += values.iter().last().unwrap();
            let mut deltas = vec![values.clone()];
            loop {
                let i = deltas.len() - 1;
                let mut new_row = Vec::new();
                let mut all_zero = true;
                for j in 0..deltas[i].len() - 1 {
                    let v = deltas[i][j + 1] - deltas[i][j];
                    if v != 0 {
                        all_zero = false;
                    }
                    new_row.push(v);
                }

                ans += new_row.iter().last().unwrap();
                deltas.push(new_row);
                if all_zero {
                    break;
                }
            }

            println!("{deltas:?}");
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;

        for values in self.values.iter() {
            let mut deltas = vec![values.clone()];
            loop {
                let i = deltas.len() - 1;
                let mut new_row = Vec::new();
                let mut all_zero = true;
                for j in 0..deltas[i].len() - 1 {
                    let v = deltas[i][j + 1] - deltas[i][j];
                    if v != 0 {
                        all_zero = false;
                    }
                    new_row.push(v);
                }

                deltas.push(new_row);
                if all_zero {
                    break;
                }
            }

            let mut pred = 0;
            for values in deltas.iter().rev() {
                pred = values[0] - pred;
            }
            println!("{values:?} => {pred}");
            ans += pred;
        }
        Ok(ans.into())
    }
}
