#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day09 {
    garbage: Vec<char>,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }

    fn scan(&self) -> (usize, usize) {
        let mut depth = 0;
        let mut score = 0;
        let mut in_garbage = false;
        let mut garbage = 0;
        for c in self.garbage.iter() {
            if in_garbage {
                if *c == '>' {
                    in_garbage = false;
                } else {
                    garbage += 1;
                }
            } else if *c == '{' {
                depth += 1;
                score += depth;
            } else if *c == '}' {
                depth -= 1;
            } else if *c == '<' {
                in_garbage = true;
            }
        }

        (score, garbage)
    }
}

impl Runner for Day09 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        let mut chars = lines[0].chars();
        while let Some(c) = chars.next() {
            if c == '!' {
                chars.next();
            } else {
                self.garbage.push(c);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.scan().0.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.scan().1.into())
    }
}
