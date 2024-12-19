#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};
use std::collections::BinaryHeap;

#[derive(Default)]
pub struct Day19 {
    available: Vec<String>,
    desired: Vec<String>,
}

impl Day19 {
    pub fn new() -> Self {
        Self::default()
    }

    fn can_make(&self, desired: &str) -> bool {
        let mut work = vec![desired];
        let mut seen = HashSet::default();

        while let Some(desired) = work.pop() {
            for available in self.available.iter() {
                if let Some(desired) = desired.strip_prefix(available) {
                    if desired.is_empty() {
                        return true;
                    }
                    if seen.insert(desired) {
                        work.push(desired);
                    }
                }
            }
        }
        false
    }

    fn ways_to_make(&self, desired: &str) -> usize {
        let mut work = BinaryHeap::new();
        work.push((desired.len(), desired));
        let mut seen = HashMap::default();
        let mut ways_to_make = 0;
        seen.insert(desired, 1);

        while let Some((_, desired)) = work.pop() {
            let cnt = *seen.get(desired).unwrap();
            for available in self.available.iter() {
                if let Some(desired) = desired.strip_prefix(available) {
                    if desired.is_empty() {
                        ways_to_make += cnt;
                    } else {
                        use std::collections::hash_map::Entry;
                        match seen.entry(desired) {
                            Entry::Vacant(entry) => {
                                entry.insert(cnt);
                                work.push((desired.len(), desired));
                            }
                            Entry::Occupied(mut entry) => {
                                *entry.get_mut() += cnt;
                            }
                        }
                    }
                }
            }
        }

        ways_to_make
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .desired
            .iter()
            .filter(|desired| self.can_make(desired))
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .desired
            .iter()
            .map(|desired| self.ways_to_make(desired))
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day19 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.available
            .extend(lines[0].split(", ").map(|s| s.into()));
        self.desired.extend(lines[2..].iter().map(|s| s.into()));
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
