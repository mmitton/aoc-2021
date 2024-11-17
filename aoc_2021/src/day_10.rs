#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day10 {
    lines: Vec<Vec<char>>,
}

impl Day10 {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

struct NavSubsystem {
    stack: Vec<char>,
}

impl NavSubsystem {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn is_valid(&mut self, line: &[char]) -> Result<usize, usize> {
        self.stack.clear();
        for c in line.iter().copied() {
            match c {
                '(' => self.stack.push(')'),
                '[' => self.stack.push(']'),
                '{' => self.stack.push('}'),
                '<' => self.stack.push('>'),
                _ => {
                    if self.stack.pop() != Some(c) {
                        match c {
                            ')' => return Err(3),
                            ']' => return Err(57),
                            '}' => return Err(1197),
                            '>' => return Err(25137),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        let mut autocomplete = 0;
        while let Some(c) = self.stack.pop() {
            autocomplete *= 5;
            autocomplete += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
        }
        Ok(autocomplete)
    }
}

impl Runner for Day10 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.lines.extend(lines.iter().map(|l| l.chars().collect()));
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
        let mut points = 0;
        let mut nav = NavSubsystem::new();
        for line in self.lines.iter() {
            if let Err(p) = nav.is_valid(line) {
                points += p;
            }
        }
        Ok(points.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut points = Vec::new();
        let mut nav = NavSubsystem::new();
        for line in self.lines.iter() {
            if let Ok(p) = nav.is_valid(line) {
                points.push(p);
            }
        }
        points.sort();
        Ok(points[points.len() / 2].into())
    }
}
