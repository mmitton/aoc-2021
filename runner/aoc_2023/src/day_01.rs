#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day01 {
    lines: Vec<String>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Runner for Day01 {
    fn parse(&mut self, path: &str) -> Result<(), Error> {
        self.lines = Lines::from_path(path, LinesOpt::RAW)?
            .iter()
            .map(|s| s.to_string())
            .collect();
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .lines
            .iter()
            .map(|line| {
                let digits: Vec<usize> = line
                    .chars()
                    .filter_map(|c| {
                        if c.is_digit(10) {
                            Some(c as usize - '0' as usize)
                        } else {
                            None
                        }
                    })
                    .collect();
                let num = (digits[0] * 10) + digits[digits.len() - 1];
                println!("{digits:?} {num}");
                num
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let digits = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
            ("zero", "0"),
        ];
        Ok(self
            .lines
            .iter_mut()
            .map(|line| {
                let mut idx = 0;
                while idx < line.len() {
                    for (from, to) in &digits {
                        if idx + from.len() > line.len() {
                            continue;
                        }
                        if &line[idx..idx + from.len()] == *from {
                            line.replace_range(idx..idx + from.len(), to)
                        }
                    }
                    idx += 1;
                }
                let digits: Vec<usize> = line
                    .chars()
                    .filter_map(|c| {
                        if c.is_digit(10) {
                            Some(c as usize - '0' as usize)
                        } else {
                            None
                        }
                    })
                    .collect();
                let num = (digits[0] * 10) + digits[digits.len() - 1];
                println!("{line} {digits:?} {num}");
                num
            })
            .sum::<usize>()
            .into())
    }
}
