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
    lines: Vec<Vec<char>>,
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
            .map(|s| s.chars().collect())
            .collect();
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        const MAP: &[(&[char], usize)] = &[
            (&['0'], 0),
            (&['1'], 1),
            (&['2'], 2),
            (&['3'], 3),
            (&['4'], 4),
            (&['5'], 5),
            (&['6'], 6),
            (&['7'], 7),
            (&['8'], 8),
            (&['9'], 9),
        ];

        let mut digits = Vec::new();
        Ok(self
            .lines
            .iter_mut()
            .map(|line| {
                let mut remaining = line.as_slice();
                digits.clear();
                'search: while !remaining.is_empty() {
                    for (from, to) in MAP.iter() {
                        if remaining.len() < from.len() {
                            continue;
                        }
                        if &remaining[0..from.len()] == *from {
                            digits.push(*to);
                            remaining = &remaining[from.len()..];
                            continue 'search;
                        }
                    }
                    remaining = &remaining[1..];
                }
                let num = (digits[0] * 10) + digits[digits.len() - 1];
                println!("{line:?} {digits:?} {num}");
                num
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        const MAP: &[(&[char], usize)] = &[
            (&['0'], 0),
            (&['1'], 1),
            (&['2'], 2),
            (&['3'], 3),
            (&['4'], 4),
            (&['5'], 5),
            (&['6'], 6),
            (&['7'], 7),
            (&['8'], 8),
            (&['9'], 9),
            (&['z', 'e', 'r', 'o'], 0),
            (&['o', 'n', 'e'], 1),
            (&['t', 'w', 'o'], 2),
            (&['t', 'h', 'r', 'e', 'e'], 3),
            (&['f', 'o', 'u', 'r'], 4),
            (&['f', 'i', 'v', 'e'], 5),
            (&['s', 'i', 'x'], 6),
            (&['s', 'e', 'v', 'e', 'n'], 7),
            (&['e', 'i', 'g', 'h', 't'], 8),
            (&['n', 'i', 'n', 'e'], 9),
        ];

        let mut digits = Vec::new();
        Ok(self
            .lines
            .iter_mut()
            .map(|line| {
                let mut remaining = line.as_slice();
                digits.clear();
                'search: while !remaining.is_empty() {
                    for (from, to) in MAP.iter() {
                        if remaining.len() < from.len() {
                            continue;
                        }
                        if &remaining[0..from.len()] == *from {
                            digits.push(*to);
                            remaining = &remaining[from.len()..];
                            continue 'search;
                        }
                    }
                    remaining = &remaining[1..];
                }
                let num = (digits[0] * 10) + digits[digits.len() - 1];
                println!("{line:?} {digits:?} {num}");
                num
            })
            .sum::<usize>()
            .into())
    }
}
