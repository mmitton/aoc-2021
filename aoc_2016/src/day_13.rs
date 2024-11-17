#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day13 {
    key: isize,
}

impl Day13 {
    pub fn new() -> Self {
        Self::default()
    }

    fn is_open(&self, x: isize, y: isize) -> bool {
        let num = (x * x) + (3 * x) + (2 * x * y) + y + (y * y);
        let num = num + self.key;

        let bits = num.count_ones();
        bits % 2 == 0
    }

    fn find_paths<F>(&self, max_len: Option<usize>, stop_at: F) -> usize
    where
        F: Fn(isize, isize) -> bool,
    {
        let mut seen = std::collections::BTreeSet::new();
        let mut paths = Vec::new();
        let max_len = max_len.unwrap_or(usize::MAX);

        let initial = (1isize, 1isize);
        paths.push(vec![initial]);
        seen.insert(initial);

        let mut i = 0;
        while i < paths.len() {
            let last = paths[i][paths[i].len() - 1];
            let x = last.0;
            let y = last.1;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dy == 0 && dx == 0) || (dy != 0 && dx != 0) {
                        continue;
                    }
                    let x = x + dx;
                    let y = y + dy;
                    if x < 0 || y < 0 {
                        continue;
                    }

                    if !self.is_open(x, y) {
                        continue;
                    }

                    let next = (x, y);
                    if seen.contains(&next) {
                        continue;
                    }

                    let mut next_path = paths[i].clone();
                    next_path.push(next);
                    if stop_at(x, y) {
                        return next_path.len() - 1;
                    }

                    seen.insert(next);
                    if next_path.len() <= max_len {
                        paths.push(next_path);
                    }
                }
            }

            i += 1;
        }

        seen.len()
    }
}

impl Runner for Day13 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.key = lines[0].parse()?;
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
        Ok(self.find_paths(None, |x, y| x == 31 && y == 39).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_paths(Some(50), |_, _| false).into())
    }
}
