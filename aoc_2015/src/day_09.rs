#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day09 {
    names: HashMap<String, usize>,
    distances: Vec<Vec<(usize, usize)>>,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_city_id(&mut self, name: &str) -> usize {
        if let Some(id) = self.names.get(name) {
            *id
        } else {
            let next = self.names.len();
            self.names.insert(name.into(), next);
            self.distances.push(Vec::new());
            next
        }
    }

    fn get_distances(&self) -> (usize, usize) {
        let mut min = usize::MAX;
        let mut max = usize::MIN;

        let mut work_queue = Vec::new();
        let total_cities = self.names.len();
        for city in 0..total_cities {
            work_queue.push((0, vec![city]));
        }
        while let Some(work) = work_queue.pop() {
            let at = work.1.last().copied().unwrap();
            for (to, dist) in self.distances[at].iter().copied() {
                if work.1.contains(&to) {
                    continue;
                }

                if work.1.len() == total_cities - 1 {
                    min = min.min(work.0 + dist);
                    max = max.max(work.0 + dist);
                } else {
                    let mut new_work = work.clone();
                    new_work.0 += dist;
                    new_work.1.push(to);
                    work_queue.push(new_work);
                }
            }
        }

        (min, max)
    }
}

impl Runner for Day09 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 5 {
                let city_a = self.get_city_id(parts[0]);
                let city_b = self.get_city_id(parts[2]);
                let dist = parts[4].parse()?;
                self.distances[city_a].push((city_b, dist));
                self.distances[city_b].push((city_a, dist));
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let (min, _) = self.get_distances();
        Ok(min.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let (_, max) = self.get_distances();
        Ok(max.into())
    }
}
