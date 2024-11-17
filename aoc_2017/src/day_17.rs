#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day17 {
    steps: usize,
}

impl Day17 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day17 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.steps = lines[0].parse()?;
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

impl Day17 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut buffer = Vec::new();
        let mut pos = 0;
        buffer.push(0);

        for iter in 1..=2017 {
            pos = (pos + self.steps) % buffer.len();

            buffer.insert(pos + 1, iter);
            pos += 1;
        }

        Ok(buffer[pos + 1].into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut pos = 0;

        let mut answer = 0;

        for iter in 1..=50000000 {
            pos = (pos + self.steps) % iter;
            if pos == 0 {
                answer = iter;
            }

            pos += 1;
        }

        Ok(answer.into())
    }
}
