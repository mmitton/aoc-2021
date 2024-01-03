use std::ops::RangeInclusive;

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day04 {
    range: RangeInclusive<u32>,
}

impl Day04 {
    pub fn new() -> Self {
        Self { range: 0..=0 }
    }

    fn passwords(&self, dup_range: RangeInclusive<u32>) -> Vec<u32> {
        let mut passwords = Vec::new();

        fn recurse(
            passwords: &mut Vec<u32>,
            num: u32,
            pos: u32,
            min: u32,
            dup_range: &RangeInclusive<u32>,
            num_dups: u32,
        ) {
            if pos == 6 {
                if num_dups > 0 {
                    passwords.push(num);
                }
                return;
            }
            if min > 9 {
                return;
            }

            for n in min..=9 {
                let mut c = n;
                let mut base = 1;
                for _ in 0..5 - pos {
                    base *= 10;
                }
                for dup in 1..=6 - pos {
                    recurse(
                        passwords,
                        num + c * base,
                        pos + dup,
                        n + 1,
                        dup_range,
                        num_dups + if dup_range.contains(&dup) { 1 } else { 0 },
                    );
                    c = (c * 10) + n;
                    base /= 10;
                }
            }
        }

        recurse(&mut passwords, 0, 0, 1, &dup_range, 0);

        passwords.retain(|v| self.range.contains(v));
        passwords
    }
}

impl Runner for Day04 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        assert!(lines.len() == 1);
        let (low, high) = lines[0].split_once('-').unwrap();
        self.range = low.parse()?..=high.parse()?;

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.passwords(2..=6).len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.passwords(2..=2).len().into())
    }
}
