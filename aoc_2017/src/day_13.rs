#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day13 {
    layers: Vec<(usize, usize, usize)>,
}

impl Day13 {
    pub fn new() -> Self {
        Self::default()
    }

    fn scanner_at(time: usize, range: usize) -> bool {
        time % range == 0
    }
}

impl Runner for Day13 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let (depth, range) = line.split_once(": ").unwrap();
            let depth: usize = depth.parse()?;
            let range: usize = range.parse()?;
            self.layers.push((depth, (range - 1) * 2, depth * range));
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

impl Day13 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .layers
            .iter()
            .copied()
            .filter_map(|(depth, range, severity)| {
                if Self::scanner_at(depth, range) {
                    Some(severity)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for layer in self.layers.iter() {
            println!("{layer:?}");
        }
        for start_at in 0.. {
            if self
                .layers
                .iter()
                .copied()
                .any(|(depth, range, _)| Self::scanner_at(start_at + depth, range))
            {
                continue;
            }
            return Ok(start_at.into());
        }
        Err(Error::Unsolved)
    }
}
