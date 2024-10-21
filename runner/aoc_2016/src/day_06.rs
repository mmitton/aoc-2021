#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day06 {
    lines: Vec<Vec<usize>>,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }

    fn scan(&self, max_needed: bool) -> String {
        let mut modified = String::with_capacity(self.lines[0].len());

        for i in 0..self.lines[0].len() {
            let mut chars = [0; 26];
            for line in self.lines.iter() {
                chars[line[i]] += 1;
            }

            let (_, ch) = if max_needed {
                chars.iter().copied().enumerate().fold(
                    (usize::MIN, 'a'),
                    |(max, max_char), (idx, cnt)| {
                        if cnt > max {
                            (cnt, (idx as u8 + b'a') as char)
                        } else {
                            (max, max_char)
                        }
                    },
                )
            } else {
                chars.iter().copied().enumerate().fold(
                    (usize::MAX, 'a'),
                    |(min, min_char), (idx, cnt)| {
                        if cnt != 0 && cnt < min {
                            (cnt, (idx as u8 + b'a') as char)
                        } else {
                            (min, min_char)
                        }
                    },
                )
            };
            modified.push(ch);
        }

        modified
    }
}

impl Runner for Day06 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.lines.extend(
            lines
                .iter()
                .map(|s| s.chars().map(|c| (c as u8 - b'a') as usize).collect()),
        );
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.scan(true).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.scan(false).into())
    }
}
