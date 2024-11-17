#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day08 {
    lines: Vec<String>,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day08 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.lines.extend(lines.iter().map(|s| s.into()));
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

impl Day08 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        fn decode_len(buffer: &mut Vec<char>, s: &str) -> usize {
            buffer.clear();
            buffer.extend(s.chars());

            let mut len = 0;
            let mut i = 1;
            while i < buffer.len() - 1 {
                len += 1;
                if buffer[i] == '\\' {
                    if buffer[i + 1] == 'x' {
                        i += 4;
                    } else {
                        i += 2;
                    }
                } else {
                    i += 1;
                }
            }

            len
        }

        let mut buffer = Vec::new();
        Ok(self
            .lines
            .iter()
            .map(|l| l.len() - decode_len(&mut buffer, l))
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        fn encode_len(buffer: &mut Vec<char>, s: &str) -> usize {
            buffer.clear();
            buffer.extend(s.chars());

            buffer.iter().filter(|c| **c == '\\' || **c == '"').count() + buffer.len() + 2
        }

        let mut buffer = Vec::new();
        Ok(self
            .lines
            .iter()
            .map(|l| encode_len(&mut buffer, l) - l.len())
            .sum::<usize>()
            .into())
    }
}
