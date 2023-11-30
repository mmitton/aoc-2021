#[allow(unused_imports)]
use crate::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Default)]
struct Elf {
    items: Vec<usize>,
    total: usize,
}

impl Elf {
    fn push(&mut self, item: usize) {
        self.total += item;
        self.items.push(item);
    }
}

pub struct Day01 {
    elves: Vec<Elf>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { elves: Vec::new() }
    }
}

impl Runner for Day01 {
    fn parse(&mut self, path: &str) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        let mut elf = Elf::default();
        for line in lines.iter() {
            if line.is_empty() {
                if elf.total != 0 {
                    self.elves.push(elf);
                    elf = Elf::default();
                }
            } else {
                let item = line.parse::<usize>()?;
                elf.push(item);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.elves.sort_by_key(|e| e.total);
        Ok(self.elves.iter().last().unwrap().total.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.elves.sort_by_key(|e| e.total);
        Ok(self
            .elves
            .iter()
            .rev()
            .take(3)
            .map(|e| e.total)
            .sum::<usize>()
            .into())
    }
}
