#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day03 {
    map: Vec<Vec<bool>>,
}

impl Day03 {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }
}

impl Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.map.push(
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    })
                    .collect(),
            );
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

impl Day03 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let width = self.map[0].len();
        Ok((0..self.map.len())
            .filter(|&y| self.map[y][(y * 3) % width])
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let width = self.map[0].len();
        Ok([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(dx, dy)| {
                (0..self.map.len())
                    .step_by(*dy)
                    .enumerate()
                    .filter(|(step, y)| self.map[*y][(step * dx) % width])
                    .count()
            })
            .product::<usize>()
            .into())
    }
}
