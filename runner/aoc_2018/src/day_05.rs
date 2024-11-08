#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
struct Polymer(Vec<char>);

impl Polymer {
    fn collapse(&mut self) {
        let mut i = 0;
        while i < self.0.len() - 1 {
            let a = self.0[i];
            let b = self.0[i + 1];
            if a.to_ascii_lowercase() == b.to_ascii_lowercase() && a != b {
                self.0.drain(i..=i + 1);
                i = i.saturating_sub(1);
                continue;
            }
            i += 1;
        }
    }
}

#[derive(Default)]
pub struct Day05 {
    polymer: Polymer,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.polymer.0.extend(lines[0].chars());
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.polymer.collapse();
        Ok(self.polymer.0.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }
}
