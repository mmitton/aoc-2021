#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day19 {
    available: Vec<String>,
    desired: Vec<String>,
}

impl Day19 {
    pub fn new() -> Self {
        Self::default()
    }

    fn towels_possible(&self) -> usize {
        let mut count = 0;
        let mut work = Vec::new();
        let mut seen = HashSet::default();
        'search: for desired in self.desired.iter() {
            work.push(desired.as_str());
            seen.clear();

            while let Some(desired) = work.pop() {
                for available in self.available.iter() {
                    if let Some(desired) = desired.strip_prefix(available) {
                        if desired.is_empty() {
                            count += 1;
                            work.clear();
                            continue 'search;
                        }
                        if seen.insert(desired) {
                            work.push(desired);
                        }
                    }
                }
            }
        }
        count
    }

    fn make_towels(&mut self) -> Vec<Option<usize>> {
        fn recurse<'a>(
            desired: &'a str,
            available: &[String],
            cache: &mut HashMap<&'a str, usize>,
        ) -> usize {
            if desired.is_empty() {
                return 1;
            }
            if let Some(value) = cache.get(desired) {
                return *value;
            }

            let count = available
                .iter()
                .filter_map(|avail| {
                    desired
                        .strip_prefix(avail)
                        .map(|desired| recurse(desired, available, cache))
                })
                .sum();
            cache.insert(desired, count);
            count
        }

        let mut cache = HashMap::default();
        (0..self.desired.len())
            .map(
                |idx| match recurse(&self.desired[idx], &self.available, &mut cache) {
                    0 => None,
                    x => Some(x),
                },
            )
            .collect()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.towels_possible().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.make_towels().iter().flatten().sum::<usize>().into())
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
