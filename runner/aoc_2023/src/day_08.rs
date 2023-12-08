use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day08 {
    instructions: Vec<char>,
    map: BTreeMap<String, (String, String)>,
}

impl Day08 {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            map: BTreeMap::new(),
        }
    }

    pub fn steps<F>(&self, from: String, to: F) -> usize
    where
        F: Fn(&str) -> bool,
    {
        let mut pos: &String = &from;
        let mut steps = 0;
        let num_inst = self.instructions.len();
        while !to(pos) {
            println!("step {steps} at {pos}");
            match self.instructions[steps % num_inst] {
                'L' => pos = &self.map.get(pos).unwrap().0,
                'R' => pos = &self.map.get(pos).unwrap().1,
                _ => unreachable!(),
            }
            steps += 1;
        }
        steps
    }
}

impl Runner for Day08 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        let lines: Vec<&str> = lines.iter().collect();
        self.instructions = lines[0].chars().collect();
        for &line in &lines[2..] {
            if let Some(line) = line.strip_suffix(')') {
                let (from, to) = line.split_once(" = (").unwrap();
                let (left, right) = to.split_once(", ").unwrap();
                self.map.insert(from.into(), (left.into(), right.into()));
            }
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.steps("AAA".into(), |pos| pos == "ZZZ").into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut starts: Vec<String> = self
            .map
            .keys()
            .filter(|k| k.ends_with('A'))
            .cloned()
            .collect();
        println!("starts: {}", starts.len());

        let cycles: Vec<usize> = starts
            .drain(..)
            .map(|start| self.steps(start, |pos| pos.ends_with('Z')))
            .collect();

        let mut ans = 1;
        for steps in cycles.iter() {
            ans = helper::lcm(ans, *steps);
        }

        println!("cycles: {cycles:?}");
        Ok(ans.into())
    }
}
