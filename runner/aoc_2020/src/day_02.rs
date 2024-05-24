#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day02 {
    policies: Vec<Policy>,
}

#[derive(Debug)]
struct Policy {
    indexes: [usize; 2],
    chars: Vec<char>,
    target: char,
}

impl Day02 {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }
}

impl Runner for Day02 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for line in lines.iter() {
            let mut parts = line.split_whitespace();
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();
            let c: Vec<char> = parts.next().unwrap().chars().collect();

            let (p1, p2) = a.split_once('-').unwrap();
            self.policies.push(Policy {
                indexes: [p1.parse::<usize>()?, p2.parse::<usize>()?],
                chars: c,
                target: b.chars().nth(0).unwrap(),
            });
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .policies
            .iter()
            .filter(|p| {
                (p.indexes[0]..=p.indexes[1])
                    .contains(&p.chars.iter().filter(|c| **c == p.target).count())
            })
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .policies
            .iter()
            .filter(|p| {
                let a = p.chars[p.indexes[0] - 1];
                let b = p.chars[p.indexes[1] - 1];
                (a == p.target || b == p.target) && a != b
            })
            .count()
            .into())
    }
}
