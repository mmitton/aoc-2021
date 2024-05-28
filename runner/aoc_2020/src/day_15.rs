#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day15 {
    starting: Vec<usize>,
}

impl Day15 {
    pub fn new() -> Self {
        Self {
            starting: Vec::new(),
        }
    }

    fn play(&self, n: u32) -> usize {
        const UNINIT: u32 = !0;
        let mut counts = vec![UNINIT; n as usize];
        let mut last = 0;
        for (i, n) in self.starting.iter().enumerate() {
            if i != self.starting.len() - 1 {
                counts[*n] = i as u32;
            }
            last = *n;
        }

        for i in self.starting.len()..n as usize {
            let last_idx = counts[last];
            let next = if last_idx != UNINIT {
                i - last_idx as usize - 1
            } else {
                0
            };
            counts[last] = i as u32 - 1;
            last = next;
        }

        last
    }
}

impl Runner for Day15 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.starting
            .extend(lines[0].split(',').map(|n| n.parse::<usize>().unwrap()));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.play(2020).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.play(30_000_000).into())
        // Ok(0.into())
    }
}
