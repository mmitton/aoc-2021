#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day17 {
    initial: Vec<[isize; 2]>,
}

impl Day17 {
    pub fn new() -> Self {
        Self {
            initial: Vec::new(),
        }
    }

    pub fn boot<const DIM: usize>(&self) -> usize {
        type D<const DIM: usize> = [isize; DIM];
        let mut cur = HashSet::default();
        let mut next = HashSet::default();

        for initial in self.initial.iter() {
            let mut c: [isize; DIM] = [0; DIM];
            c[0] = initial[0];
            c[1] = initial[1];
            cur.insert(c);
        }

        cur.len()
    }
}

impl Runner for Day17 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for (y, row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    self.initial.push([x, y])
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }
}
