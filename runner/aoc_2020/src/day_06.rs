#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
struct Group {
    answers: [usize; 26],
    people: usize,
}

pub struct Day06 {
    groups: Vec<Group>,
}

impl Day06 {
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }
}

impl Runner for Day06 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut group = Group::default();
        for line in lines.iter() {
            if line.is_empty() {
                if group.people > 0 {
                    self.groups.push(group);
                    group = Group::default();
                }
            } else {
                group.people += 1;
                line.chars()
                    .for_each(|c| group.answers[(c as u8 - b'a') as usize] += 1);
            }
        }
        if group.people > 0 {
            self.groups.push(group);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .groups
            .iter()
            .map(|g| g.answers.iter().filter(|a| **a > 0).count())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .groups
            .iter()
            .map(|g| g.answers.iter().filter(|a| **a == g.people).count())
            .sum::<usize>()
            .into())
    }
}
