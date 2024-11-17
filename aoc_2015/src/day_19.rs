#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day19 {
    molecule: String,
    rules: Vec<(String, String)>,
}

impl Day19 {
    pub fn new() -> Self {
        Self::default()
    }

    fn expand(&self) -> usize {
        let mut outputs = HashSet::default();
        for (from, to) in self.rules.iter() {
            let rule_len = from.len();
            for i in 0..self.molecule.len() - rule_len + 1 {
                if self.molecule[i..i + rule_len] == *from {
                    let mut output = self.molecule[0..i].to_string();
                    output.push_str(to);
                    output.push_str(&self.molecule[i + rule_len..]);
                    outputs.insert(output);
                }
            }
        }
        outputs.len()
    }

    fn reduce(&self) -> usize {
        let mut molecule = self.molecule.clone();
        let mut steps = 0;
        while molecule != "e" {
            for (from, to) in self.rules.iter() {
                if from == "e" {
                    if &molecule == to {
                        steps += 1;
                        molecule = from.into();
                        break;
                    }
                } else {
                    let new_molecule = molecule.replacen(to, from, 1);
                    if new_molecule != molecule {
                        molecule = new_molecule;
                        steps += 1;
                    }
                }
            }
        }
        steps
    }
}

impl Runner for Day19 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        for line in lines.iter() {
            if let Some((from, to)) = line.split_once(" => ") {
                self.rules.push((from.into(), to.into()));
            } else {
                self.molecule.push_str(line);
            }
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

impl Day19 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.expand().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.reduce().into())
    }
}
