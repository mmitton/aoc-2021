#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

struct Program {
    pipes: Vec<usize>,
    group: usize,
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" <-> ").collect();

        let mut pipes = Vec::new();
        for pipe in parts[1].split(", ") {
            pipes.push(pipe.parse()?);
        }

        Ok(Program { pipes, group: 0 })
    }
}

#[derive(Default)]
pub struct Day12 {
    programs: Vec<Program>,
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }

    fn map_group(&mut self, idx: usize, group: usize) -> Result<usize, Error> {
        if self.programs[idx].group != 0 {
            return Err(Error::Runner(format!(
                "Cannot remap program {idx} to group {group}.  Already set to {}",
                self.programs[idx].group
            )));
        }

        let mut count = 1;
        self.programs[idx].group = group;

        for to in self.programs[idx].pipes.clone().iter().copied() {
            match self.programs[to].group {
                0 => count += self.map_group(to, group)?,
                to_group => {
                    if to_group != group {
                        unreachable!();
                    }
                }
            }
        }

        Ok(count)
    }
}

impl Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;

        for (idx, line) in lines.iter().enumerate() {
            assert_eq!(idx, self.programs.len());
            self.programs.push(line.parse()?);
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

impl Day12 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.map_group(0, 1)?.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut group = 0;
        while let Some(idx) = self.programs.iter().position(|p| p.group == 0) {
            group += 1;
            self.map_group(idx, group)?;
        }
        Ok(group.into())
    }
}
