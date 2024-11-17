#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(PartialEq, Eq)]
struct Word {
    letters: [u8; 26],
    chars: Vec<char>,
}

impl FromStr for Word {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut letters = [0; 26];
        let mut chars = Vec::new();
        for c in s.chars() {
            if !c.is_ascii_lowercase() {
                return Err(Error::InvalidInput(s.into()));
            }
            letters[(c as u8 - b'a') as usize] += 1;
            chars.push(c);
        }
        Ok(Self { letters, chars })
    }
}

#[derive(Default)]
pub struct Day02 {
    words: Vec<Word>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.words.push(line.parse()?);
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

impl Day02 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let (two, three) = self
            .words
            .iter()
            .fold((0, 0), |(mut two, mut three), word| {
                if word.letters.iter().any(|c| *c == 2) {
                    two += 1;
                }
                if word.letters.iter().any(|c| *c == 3) {
                    three += 1;
                }
                (two, three)
            });

        Ok((two * three).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut common: Vec<char> = Vec::new();
        for w1 in self.words.iter() {
            for w2 in self.words.iter() {
                if w1 == w2 {
                    continue;
                }
                common.clear();
                common.extend(w1.chars.iter().zip(w2.chars.iter()).filter_map(|(a, b)| {
                    if a == b {
                        Some(a)
                    } else {
                        None
                    }
                }));
                if common.len() == w1.chars.len() - 1 {
                    return Ok(String::from_iter(common).into());
                }
            }
        }
        Err(Error::Unsolved)
    }
}
