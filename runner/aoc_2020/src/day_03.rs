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
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
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
