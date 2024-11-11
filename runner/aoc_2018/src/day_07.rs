#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct Day07 {
    steps: Vec<Vec<usize>>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }

    fn complete(&self, workers: usize, time_base: usize) -> (String, usize) {
        let mut order = String::new();
        let mut started: HashSet<usize> = HashSet::default();
        let mut done: HashSet<usize> = HashSet::default();
        let mut work: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut workers_avail = workers;
        let mut steps_avail: BTreeSet<usize> = self
            .steps
            .iter()
            .enumerate()
            .filter_map(|(step, depends_on)| {
                if depends_on.is_empty() {
                    Some(step)
                } else {
                    None
                }
            })
            .collect();
        work.insert(0, Vec::new());

        let mut max_time = 0;
        while let Some((time, steps_done)) = work.pop_first() {
            workers_avail += steps_done.len();
            max_time = time;
            for step in steps_done.iter() {
                done.insert(*step);
            }
            'avail_check: for (step, depends_on) in self.steps.iter().enumerate() {
                if done.contains(&step) || started.contains(&step) {
                    continue;
                }
                for depends_on in depends_on.iter() {
                    if !done.contains(depends_on) {
                        continue 'avail_check;
                    }
                }
                steps_avail.insert(step);
            }

            while workers_avail != 0 {
                if let Some(step) = steps_avail.pop_first() {
                    let done_at = time + time_base + step + 1;
                    started.insert(step);
                    work.entry(done_at).or_default().push(step);
                    order.push((step as u8 + b'A') as char);
                    workers_avail -= 1;
                } else {
                    break;
                }
            }
        }
        (order, max_time)
    }
}

impl Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some(line) = line.strip_prefix("Step ") {
                let from = (line.chars().next().unwrap() as u8 - b'A') as usize;
                if let Some(line) = line[1..].strip_prefix(" must be finished before step ") {
                    let to = (line.chars().next().unwrap() as u8 - b'A') as usize;
                    while self.steps.len() <= to {
                        self.steps.push(Vec::new());
                    }
                    self.steps[to].push(from);
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.complete(1, 0).0.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let (workers, time_base) = if self.steps.len() == 6 {
            (2, 0)
        } else {
            (5, 60)
        };
        Ok(self.complete(workers, time_base).1.into())
    }
}
