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
        let mut builder: BTreeMap<usize, Vec<(u32, usize)>> = BTreeMap::new();
        let mut found = BitArray::new(1 << self.packages.len());

        for (i, weight) in self.packages.iter().enumerate() {
            builder.entry(*weight).or_default().push((1 << i, *weight));
            found.set(1 << i, true);
        }

        while let Some((total_weight, arrangements)) = builder.pop_first() {
            if total_weight == compartment_weight {
                return Vec::from_iter(arrangements.iter().copied());
            }

            for (i, weight) in self.packages.iter().enumerate() {
                let new_weight = total_weight + weight;
                if new_weight > compartment_weight {
                    continue;
                }
                let mask = 1 << i;
                for (packages, qe) in arrangements.iter() {
                    if packages & mask == 0 && !found.set((packages | mask) as usize, true) {
                        builder
                            .entry(new_weight)
                            .or_default()
                            .push((packages | mask, qe.saturating_mul(*weight)));
                    }
                }
            }
        }

        Vec::new()
    }

    fn pack_sleigh(
        &self,
        passenger: u32,
        arrangements: &[(u32, usize)],
        compartments: usize,
    ) -> bool {
        let mut work = Vec::new();
        work.push((1, passenger));

        while let Some((cnt, packed)) = work.pop() {
            for (packages, _) in arrangements.iter() {
                if packed & packages == 0 {
                    if cnt + 1 == compartments {
                        return (packed | packages).count_ones() as usize == self.packages.len();
                    }
                    work.push((cnt + 1, packed | packages))
                }
            }
        }
        false
    }

    fn run(&mut self, compartments: usize) -> usize {
        let arrangements = self.calc_arrangements(compartments);
        println!(
            "Found {} arrangements for {} compartments",
            arrangements.len(),
            compartments
        );

        for i in 0..self.packages.len() as u32 {
            let starter: Vec<(u32, usize)> = arrangements
                .iter()
                .copied()
                .filter(|(packages, _)| packages.count_ones() == i)
                .collect();

            let mut best = usize::MAX;
            for (packages, qe) in starter.iter().copied() {
                if self.pack_sleigh(packages, &arrangements, compartments) {
                    best = best.min(qe);
                }
            }
            if best != usize::MAX {
                return best;
            }
        }
        0
    }
}

impl Runner for Day24 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.packages.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.run(3).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.run(4).into())
    }
}
