#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day10 {
    x_vals: Vec<isize>,
}

impl Day10 {
    pub fn new() -> Self {
        Self { x_vals: Vec::new() }
    }
}

impl Runner for Day10 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut x = 1;
        self.x_vals.push(x);
        for line in lines.iter() {
            self.x_vals.push(x);
            if let Some((op, delta)) = line.split_once(' ') {
                assert_eq!(op, "addx");
                x += delta.parse::<isize>()?;
                self.x_vals.push(x);
            } else {
                assert_eq!(line, "noop");
            }
        }
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
        Ok(self
            .x_vals
            .iter()
            .enumerate()
            .skip(19)
            .step_by(40)
            .map(|(cycle, x)| (cycle as isize + 1) * x)
            .sum::<isize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut result = String::new();
        for c in 1..=240 {
            let crt = (c - 1) % 40;
            let cycle = self.x_vals[c - 1];

            let diff = (cycle - crt as isize).abs();
            if diff <= 1 {
                result.push('#');
            } else {
                result.push(' ');
            }
            if c % 40 == 0 {
                result.push('\n');
            }
        }
        Ok(result.into())
    }
}
