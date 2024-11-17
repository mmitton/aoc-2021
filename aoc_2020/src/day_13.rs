#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day13 {
    start_time: usize,
    busses: Vec<(usize, usize)>,
}

impl Day13 {
    pub fn new() -> Self {
        Self {
            start_time: usize::MAX,
            busses: Vec::new(),
        }
    }
}

impl Runner for Day13 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 2);
        self.start_time = lines[0].parse()?;
        self.busses
            .extend(lines[1].split(',').enumerate().filter_map(|(i, b)| {
                if let Ok(b) = b.parse() {
                    Some((i, b))
                } else {
                    None
                }
            }));
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
        let mut best = (usize::MAX, usize::MAX);
        for (_, b) in self.busses.iter() {
            let pickup = ((self.start_time + (b - 1)) / b) * b;
            if pickup - self.start_time < best.0 {
                best.0 = pickup - self.start_time;
                best.1 = *b;
            }
        }
        Ok((best.0 * best.1).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut max_idx = 0;
        let mut max_bus = 0;
        for bus in self.busses.iter() {
            if bus.1 > max_bus {
                max_bus = bus.1;
                max_idx = bus.0;
            }
        }

        let mut step = max_bus;
        for bus in self.busses.iter() {
            if (bus.0 as isize - max_idx as isize).unsigned_abs() == bus.1 {
                step *= bus.1;
            }
        }

        'start_loop: for start in (step..usize::MAX).step_by(step) {
            for i in 0..self.busses.len() {
                let trips =
                    (start + self.busses[i].1 + self.busses[i].0 - max_idx - 1) / self.busses[i].1;
                let arrive_at = trips * self.busses[i].1;
                if arrive_at != start + self.busses[i].0 - max_idx {
                    continue 'start_loop;
                }
            }

            let start = start - max_idx;
            return Ok(start.into());
        }
        Err(Error::Unsolved)
    }
}
