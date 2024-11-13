#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day06 {
    fish: Vec<u8>,
}

impl Day06 {
    pub fn new() -> Self {
        Self { fish: Vec::new() }
    }

    fn after(&self, days: usize) -> usize {
        fn spawn(cache: &mut HashMap<usize, usize>, day: usize, days: usize) -> usize {
            // println!("spawn({}, {})", day, days);
            let mut total = 0usize;
            if day <= days {
                if let Some(day_total) = cache.get(&day) {
                    total += *day_total;
                } else {
                    let day_total = 1 + spawn(cache, day + 7, days) + spawn(cache, day + 9, days);
                    cache.insert(day, day_total);
                    total += day_total;
                }
            }

            total
        }

        let mut cache: HashMap<usize, usize> = HashMap::default();
        let mut total = self.fish.len();
        for fish in &self.fish {
            total += spawn(&mut cache, *fish as usize + 1, days);
        }

        total
    }
}

impl Runner for Day06 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.fish
            .extend(lines[0].split(',').map(|n| n.parse::<u8>().unwrap()));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.after(80).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.after(256).into())
    }
}
