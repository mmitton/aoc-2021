#[allow(unused_imports)]
use helper::{
    print, println, BitArray, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner,
};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Day24 {
    packages: Vec<usize>,
}

impl Day24 {
    pub fn new() -> Self {
        Self::default()
    }

    fn calc_arrangements(&self, compartments: usize) -> Vec<(u32, usize)> {
        let compartment_weight = self.packages.iter().sum::<usize>() / compartments;
        let mut builder: BTreeMap<usize, Vec<(u32, usize, usize)>> = BTreeMap::new();
        let mut found = BitArray::new(1 << self.packages.len());

        for (i, weight) in self.packages.iter().enumerate() {
            builder
                .entry(1)
                .or_default()
                .push((1 << i, *weight, *weight));
            found.set(1 << i, true);
        }

        let mut ans = Vec::new();
        while let Some((num_packages, arrangements)) = builder.pop_first() {
            for (i, weight) in self.packages.iter().enumerate() {
                let mask = 1 << i;
                for (packages, qe, total_weight) in arrangements.iter() {
                    let new_weight = total_weight + weight;
                    if new_weight > compartment_weight {
                        continue;
                    }
                    if packages & mask == 0 && !found.set((packages | mask) as usize, true) {
                        if new_weight == compartment_weight {
                            ans.push((packages | mask, qe.saturating_mul(*weight)));
                        } else {
                            builder.entry(num_packages + 1).or_default().push((
                                packages | mask,
                                qe.saturating_mul(*weight),
                                new_weight,
                            ));
                        }
                    }
                }
            }

            if !ans.is_empty() {
                return ans;
            }
        }

        Vec::new()
    }

    fn run(&mut self, compartments: usize) -> usize {
        let arrangements = self.calc_arrangements(compartments);
        arrangements
            .iter()
            .copied()
            .map(|(_, qe)| qe)
            .min()
            .unwrap()
    }
}

impl Runner for Day24 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.packages.push(line.parse()?);
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

impl Day24 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.run(3).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.run(4).into())
    }
}
