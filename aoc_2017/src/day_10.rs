use crate::knot_hash::KnotHash;
#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::fmt::Write;

#[derive(Default)]
pub struct Day10 {
    knot_hash: KnotHash,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day10 {
    fn parse(&mut self, file: &[u8], part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        if part1 {
            assert_eq!(lines.len(), 1);
            for n in lines[0].split(',') {
                self.knot_hash.lengths.push(n.parse()?);
            }

            if self.knot_hash.lengths.len() == 4 {
                self.knot_hash.set_numbers_len(4);
            }
        } else if lines.len() == 1 {
            self.knot_hash.extent_from_str(&lines[0]);
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.knot_hash.reorder(1);
        Ok(self
            .knot_hash
            .numbers
            .iter()
            .take(2)
            .map(|v| *v as usize)
            .product::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let hash = self.knot_hash.hash();
        let mut hash_str = String::with_capacity(32);
        for h in hash {
            write!(&mut hash_str, "{:02x}", h)?;
        }
        Ok(hash_str.into())
    }
}
