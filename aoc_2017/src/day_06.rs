#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day06 {
    banks: Vec<usize>,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }

    fn realloc(&mut self) {
        let max_idx = self
            .banks
            .iter()
            .copied()
            .enumerate()
            .fold(
                (0, usize::MIN),
                |(idx, max), (i, v)| if v > max { (i, v) } else { (idx, max) },
            )
            .0;

        let mut left = self.banks[max_idx];
        self.banks[max_idx] = 0;
        let mut idx = max_idx + 1;
        let num_banks = self.banks.len();
        while left > 0 {
            self.banks[idx % num_banks] += 1;
            idx += 1;
            left -= 1;
        }
    }

    fn find_cycle(&mut self) -> (usize, usize) {
        let mut banks = HashMap::default();
        banks.insert(self.banks.clone(), 0);
        for steps in 1.. {
            self.realloc();

            let prev = *banks.entry(self.banks.clone()).or_insert(steps);
            if prev != steps {
                return (steps, steps - prev);
            }
        }
        (0, 0)
    }
}

impl Runner for Day06 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        for v in lines[0].split_whitespace() {
            self.banks.push(v.parse()?);
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

impl Day06 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_cycle().0.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_cycle().1.into())
    }
}
