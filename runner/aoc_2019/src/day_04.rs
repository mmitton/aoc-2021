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

    fn passwords(&self, dup_range: RangeInclusive<u32>) -> u32 {
        fn recurse(
            num: u32,
            pos: u32,
            min: u32,
            dup_range: &RangeInclusive<u32>,
            num_dups: u32,
            range: &RangeInclusive<u32>,
        ) -> u32 {
            if pos == 6 {
                if num_dups > 0 && range.contains(&num) {
                    println!("{num}");
                    return 1;
                }
                return 0;
            }
            if min > 9 {
                return 0;
            }

            let mut passwords = 0;

            for n in min..=9 {
                let mut c = n;
                let mut base = 1;
                for _ in 0..5 - pos {
                    base *= 10;
                }
                for dup in 1..=6 - pos {
                    passwords += recurse(
                        num + c * base,
                        pos + dup,
                        n + 1,
                        dup_range,
                        num_dups + if dup_range.contains(&dup) { 1 } else { 0 },
                        range,
                    );
                    c = (c * 10) + n;
                    base /= 10;
                }
            }

            passwords
        }

        recurse(0, 0, 1, &dup_range, 0, &self.range)
    }
}

impl Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert!(lines.len() == 1);
        let (low, high) = lines[0].split_once('-').unwrap();
        self.range = low.parse()?..=high.parse()?;

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.passwords(2..=6).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.passwords(2..=2).into())
    }
}
