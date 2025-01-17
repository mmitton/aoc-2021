use std::ops::RangeInclusive;

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day04 {
    elves: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>,
}

impl Day04 {
    pub fn new() -> Self {
        Self { elves: Vec::new() }
    }
}

impl Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let (elf1, elf2) = line.split_once(',').unwrap();

            macro_rules! range {
                ($range:expr) => {{
                    let (lo, hi) = $range.split_once('-').unwrap();
                    let lo: usize = lo.parse().unwrap();
                    let hi: usize = hi.parse().unwrap();
                    lo..=hi
                }};
            }
            let elf1 = range!(elf1);
            let elf2 = range!(elf2);
            self.elves.push((elf1, elf2));
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

impl Day04 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .elves
            .iter()
            .map(|(elf1, elf2)| {
                if elf1.contains(elf2.start()) && elf1.contains(elf2.end())
                    || elf2.contains(elf1.start()) && elf2.contains(elf1.end())
                {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .elves
            .iter()
            .map(|(elf1, elf2)| {
                if elf1.contains(elf2.start())
                    || elf1.contains(elf2.end())
                    || elf2.contains(elf1.start())
                    || elf2.contains(elf1.end())
                {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>()
            .into())
    }
}
