#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeSet;

use crate::knot_hash::KnotHash;

#[derive(Default)]
pub struct Day14 {
    key: String,
    blocks: BTreeSet<(u8, u8)>,
}

impl Day14 {
    pub fn new() -> Self {
        Self::default()
    }

    fn map_disk(&mut self) -> Result<usize, Error> {
        for y in 0..128 {
            let mut knot_hash: KnotHash = format!("{}-{y}", self.key).parse()?;
            let hash = knot_hash.hash();
            for (x, v) in hash.iter().enumerate() {
                for x_offset in 0..8 {
                    if v & (1 << (7 - x_offset)) != 0 {
                        assert!(self.blocks.insert((x as u8 * 8 + x_offset, y)));
                    }
                }
                if x == 0 && y == 0 {
                    println!("{v:08b} {:?}", self.blocks);
                }
            }
        }

        Ok(self.blocks.len())
    }

    fn regions(&mut self) -> usize {
        let mut regions = 0;
        let mut work = Vec::new();
        while let Some(block) = self.blocks.pop_first() {
            regions += 1;

            work.push(block);
            while let Some(block) = work.pop() {
                for dir in [
                    (block.0.wrapping_sub(1), block.1),
                    (block.0.wrapping_add(1), block.1),
                    (block.0, block.1.wrapping_sub(1)),
                    (block.0, block.1.wrapping_add(1)),
                ] {
                    if self.blocks.remove(&dir) {
                        work.push(dir);
                    }
                }
            }
        }
        regions
    }
}

impl Runner for Day14 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.key.push_str(&lines[0]);
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.map_disk()?.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.map_disk()?;
        Ok(self.regions().into())
    }
}
