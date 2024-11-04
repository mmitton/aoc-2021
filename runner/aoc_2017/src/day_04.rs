#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day04 {
    passphrases: Vec<Vec<String>>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }

    fn is_valid(passphrase: &[String], anagrams: bool) -> bool {
        fn char_counts(s: &str) -> HashMap<char, usize> {
            let mut counts = HashMap::default();
            for c in s.chars() {
                *counts.entry(c).or_default() += 1;
            }
            counts
        }

        let char_counts: Vec<HashMap<char, usize>> =
            passphrase.iter().map(|s| char_counts(s)).collect();

        for (i, (a, a_counts)) in passphrase.iter().zip(char_counts.iter()).enumerate() {
            for (b, b_counts) in passphrase.iter().zip(char_counts.iter()).skip(i + 1) {
                if a == b {
                    return false;
                }
                if anagrams && a_counts == b_counts {
                    return false;
                }
            }
        }
        true
    }
}

impl Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.passphrases
                .push(line.split_whitespace().map(|s| s.into()).collect());
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .passphrases
            .iter()
            .filter(|p| Self::is_valid(p, false))
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .passphrases
            .iter()
            .filter(|p| Self::is_valid(p, true))
            .count()
            .into())
    }
}
