#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone, Default)]
struct Polymer(Vec<char>);

impl Polymer {
    fn collapse(&mut self) {
        let mut new_polymer = Vec::with_capacity(self.0.len());
        let mut last: char = ' ';
        for c in self.0.iter().copied() {
            if new_polymer.is_empty() {
                new_polymer.push(c);
                last = c;
            } else if last.to_ascii_lowercase() == c.to_ascii_lowercase() && last != c {
                new_polymer.truncate(new_polymer.len() - 1);
                last = new_polymer.last().copied().unwrap_or(' ');
            } else {
                new_polymer.push(c);
                last = c;
            }
        }

        std::mem::swap(&mut new_polymer, &mut self.0);
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
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.polymer.0.extend(lines[0].chars());
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

impl Day05 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.polymer.collapse();
        Ok(self.polymer.0.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut best = usize::MAX;
        for (lc, uc) in ('a'..='z').zip('A'..='Z') {
            let mut polymer = Polymer(
                self.polymer
                    .0
                    .iter()
                    .copied()
                    .filter(|c| *c != lc && *c != uc)
                    .collect(),
            );
            polymer.collapse();
            best = best.min(polymer.0.len());
        }
        Ok(best.into())
    }
}
