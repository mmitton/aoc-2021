#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::ops::RangeInclusive;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Blacklist {
    lo: u32,
    hi: u32,
}

impl Blacklist {
    fn as_range(&self) -> RangeInclusive<u32> {
        self.lo..=self.hi
    }
}

#[derive(Default)]
pub struct Day20 {
    blacklists: Vec<Blacklist>,
}

impl Day20 {
    pub fn new() -> Self {
        Self::default()
    }

    fn sort_and_merge(&mut self) {
        let mut blacklists: Vec<Blacklist> = Vec::new();
        self.blacklists.sort();
        for blacklist in self.blacklists.iter() {
            if let Some(last) = blacklists.last_mut() {
                if last.hi == u32::MAX || last.hi + 1 >= blacklist.lo {
                    if last.hi < blacklist.hi {
                        last.hi = blacklist.hi;
                    }
                } else {
                    blacklists.push(*blacklist);
                }
            } else {
                blacklists.push(*blacklist);
            }
        }
        std::mem::swap(&mut blacklists, &mut self.blacklists);
    }
}

impl Runner for Day20 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some((lo, hi)) = line.split_once('-') {
                let lo = lo.parse()?;
                let hi = hi.parse()?;
                self.blacklists.push(Blacklist { lo, hi });
            }
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.sort_and_merge();
        let mut lowest = 0u32;
        for blacklist in self.blacklists.iter() {
            if blacklist.as_range().contains(&lowest) {
                lowest = blacklist.hi + 1;
            } else {
                break;
            }
        }
        Ok(lowest.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.sort_and_merge();
        let mut valid = self
            .blacklists
            .windows(2)
            .map(|pairs| pairs[1].lo - pairs[0].hi - 1)
            .sum::<u32>();

        if let Some(first) = self.blacklists.first() {
            valid += first.lo;
        }
        if let Some(last) = self.blacklists.last() {
            valid += u32::MAX - last.hi;
        }

        Ok(valid.into())
    }
}
