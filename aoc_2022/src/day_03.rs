#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day03 {
    lines: Vec<Vec<char>>,
}

impl Day03 {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        for line in Lines::from_bufread(file, LinesOpt::TRIM)?.iter() {
            self.lines.push(line.chars().collect());
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

impl Day03 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .lines
            .iter()
            .map(|line| {
                let split = line.len() / 2;
                let left = &line[..split];
                let right = &line[split..];
                for c in left.iter().copied() {
                    if right.contains(&c) {
                        println!("Left:{left:?}  Right:{right:?}  In both: {c}");
                        return match c {
                            'a'..='z' => (c as usize - 'a' as usize) + 1,
                            'A'..='Z' => (c as usize - 'A' as usize) + 27,
                            _ => unreachable!("Invalid char '{c}'"),
                        };
                    }
                }
                unreachable!("Did not find a match");
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .lines
            .chunks(3)
            .map(|lines| {
                for c in lines[0].iter().copied() {
                    if lines[1].contains(&c) && lines[2].contains(&c) {
                        println!("Lines: {lines:?}  In all: {c}");
                        return match c {
                            'a'..='z' => (c as usize - 'a' as usize) + 1,
                            'A'..='Z' => (c as usize - 'A' as usize) + 27,
                            _ => unreachable!("Invalid char '{c}'"),
                        };
                    }
                }
                unreachable!("Did not find a match");
            })
            .sum::<usize>()
            .into())
    }
}
