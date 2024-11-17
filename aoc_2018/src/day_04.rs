#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day04 {
    guards: HashMap<usize, Vec<(usize, usize)>>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines: Vec<&str> = lines.iter().collect();
        lines.sort();

        let mut guard: usize = 0;
        let mut sleep_time: usize = 0;

        for line in lines {
            if line.contains("begins shift") {
                let parts: Vec<&str> = line.split(" ").collect();
                guard = parts[3][1..].parse()?;
            } else if line.contains("falls") {
                sleep_time = line[15..17].parse()?;
            } else if line.contains("wakes") {
                let wake_time: usize = line[15..17].parse()?;

                let sleep_times = self.guards.entry(guard).or_default();
                sleep_times.push((sleep_time, wake_time));
            }
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

impl Day04 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut max_sleep = 0;
        let mut max_guard = 0;
        let mut max_min = 0;

        for (guard, sleep_times) in self.guards.iter() {
            let mut minutes = vec![0; 60];

            let mut guard_max_cnt = 0;
            let mut guard_max_min = 0;
            let mut guard_sleep = 0;

            for sleep_time in sleep_times {
                #[allow(clippy::needless_range_loop)]
                for min in sleep_time.0..sleep_time.1 {
                    guard_sleep += 1;
                    minutes[min] += 1;
                    if minutes[min] > guard_max_cnt {
                        guard_max_cnt = minutes[min];
                        guard_max_min = min;
                    }
                }
            }

            if guard_sleep > max_sleep {
                max_sleep = guard_sleep;
                max_guard = *guard;
                max_min = guard_max_min;
            }
        }

        Ok((max_min * max_guard).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut max_cnt = 0;
        let mut max_guard = 0;
        let mut max_min = 0;

        for (guard, sleep_times) in self.guards.iter() {
            let mut minutes = vec![0; 60];

            for sleep_time in sleep_times {
                #[allow(clippy::needless_range_loop)]
                for min in sleep_time.0..sleep_time.1 {
                    minutes[min] += 1;
                    if minutes[min] > max_cnt {
                        max_cnt = minutes[min];
                        max_min = min;
                        max_guard = *guard;
                    }
                }
            }
        }
        Ok((max_min * max_guard).into())
    }
}
