#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::cmp::Ordering;

#[derive(Default)]
pub struct Day17 {
    target: u32,
    jugs: Vec<u32>,
}

impl Day17 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find_combinations<F>(&self, mut f: F) -> usize
    where
        F: FnMut(u32) -> (bool, u32),
    {
        let mut visited: HashSet<u32> = HashSet::default();
        let mut good = 0;
        let mut explore: Vec<(u32, u32)> = Vec::new();
        let mut max_jugs = u32::BITS;

        for i in 0..self.jugs.len() {
            if self.jugs[i] <= self.target {
                visited.insert(1 << i);
                explore.push((self.jugs[i], 1 << i));
            }
        }

        while let Some((cap, jugs)) = explore.pop() {
            // println!("{}: {:032b}", cap, jugs);
            for (i, jug) in self.jugs.iter().enumerate() {
                let mask = 1 << i;
                if jugs & mask != 0 {
                    continue;
                }

                if cap + jug > self.target {
                    continue;
                }

                let new_jugs = jugs | mask;
                if new_jugs.count_ones() > max_jugs {
                    continue;
                }
                if visited.insert(new_jugs) {
                    if cap + jug == self.target {
                        let (is_good, new_max_jugs) = f(new_jugs);
                        if is_good {
                            if new_max_jugs != max_jugs {
                                max_jugs = new_max_jugs;
                                good = 1;
                            } else {
                                good += 1;
                            }
                        }
                    } else {
                        explore.push((cap + jug, new_jugs));
                    }
                }
            }
        }

        good
    }
}

impl Runner for Day17 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.jugs.push(line.parse()?);
        }
        self.target = if self.jugs.len() == 5 { 25 } else { 150 };
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_combinations(|_| (true, u32::BITS)).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut min_jugs = u32::BITS;
        Ok(self
            .find_combinations(|jugs| {
                let jugs_count = jugs.count_ones();
                match jugs_count.cmp(&min_jugs) {
                    Ordering::Less => {
                        min_jugs = jugs_count;
                        (true, min_jugs)
                    }
                    Ordering::Equal => (true, min_jugs),
                    Ordering::Greater => (false, min_jugs),
                }
            })
            .into())
    }
}
