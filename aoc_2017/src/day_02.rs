#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day02 {
    rows: Vec<Vec<usize>>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let row = line
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect();
            self.rows.push(row);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut checksum = 0;
        for row in self.rows.iter() {
            let mut min = usize::MAX;
            let mut max = usize::MIN;
            for v in row.iter().copied() {
                min = min.min(v);
                max = max.max(v);
            }
            checksum += max - min;
        }
        Ok(checksum.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut sums = 0;
        'rows: for row in self.rows.iter() {
            for (i, a) in row.iter().enumerate() {
                for b in row.iter().skip(i + 1) {
                    let (num, div) = if a > b { (a, b) } else { (b, a) };
                    if num % div == 0 {
                        sums += num / div;
                        continue 'rows;
                    }
                }
            }
            unreachable!();
        }
        Ok(sums.into())
    }
}
