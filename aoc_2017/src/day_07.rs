use std::{fmt::Write, str::FromStr};

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone, Debug)]
struct Program {
    name: String,
    parent: String,
    weight: isize,
    holding: Vec<Program>,
    holding_names: Vec<String>,
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let name_weight = parts[0].replace(" (", " ");
        let name_weight = name_weight.replace(")", "");
        let name_weight: Vec<&str> = name_weight.split(" ").collect();
        let mut program = Program {
            name: name_weight[0].to_string(),
            parent: String::new(),
            weight: name_weight[1].parse()?,
            holding: Vec::new(),
            holding_names: Vec::new(),
        };

        if parts.len() == 2 {
            for holding in parts[1].split(", ") {
                program.holding_names.push(holding.to_string());
            }
        }

        Ok(program)
    }
}

impl Program {
    fn weight(&self) -> isize {
        let mut weight = self.weight;
        for h in &self.holding {
            weight += h.weight();
        }

        weight
    }

    fn weights(&self) -> HashMap<isize, usize> {
        let mut weights = HashMap::default();
        for h in &self.holding {
            let w = h.weight();
            let num = *weights.get(&w).unwrap_or(&0) + 1;
            weights.insert(w, num);
        }

        weights
    }

    fn fix_weight(&mut self, delta: isize) -> isize {
        let weights = self.weights();

        if weights.len() < 2 {
            // I must be the problem
            self.weight += delta;
            return self.weight;
        }

        // Must be a child
        assert!(weights.len() == 2);
        let mut good_weight = 0isize;
        let mut bad_weight = 0isize;
        for (weight, num) in weights.iter() {
            if *num == 1 {
                bad_weight = *weight;
            } else {
                good_weight = *weight;
            }
        }

        let delta = good_weight - bad_weight;

        for h in &mut self.holding {
            if h.weight() == bad_weight {
                return h.fix_weight(delta);
            }
        }

        panic!("wtf");
    }
}

#[derive(Default)]
pub struct Day07 {
    programs: HashMap<String, Program>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_root(&mut self) -> Result<Program, Error> {
        let programs: Vec<Program> = self.programs.values().cloned().collect();
        for program in programs.iter() {
            for child in program.holding_names.iter() {
                if let Some(child) = self.programs.get_mut(child) {
                    if !child.parent.is_empty() {
                        return Err(Error::Runner(format!(
                        "Tried to set parent name on {:?} to {:?}.  Parent name already set to {:?}",
                        child.name, child.parent, program.name
                    )));
                    }
                    child.parent.write_str(program.name.as_str())?;
                } else {
                    return Err(Error::Runner(format!(
                        "Tried to set parent name on {:?} to {:?}.  {:?} cannot be found",
                        child, program.name, child
                    )));
                }
            }
        }

        while self.programs.len() > 1 {
            for program in programs.iter() {
                if let Some(program) = self.programs.remove(program.name.as_str()) {
                    if program.holding.len() == program.holding_names.len() {
                        if let Some(parent) = self.programs.get_mut(program.parent.as_str()) {
                            parent.holding.push(program);
                        } else {
                            return Err(Error::Runner(format!(
                                "Tried to add child {:?} to parent {:?}.  Could not find parent",
                                program.name, program.parent
                            )));
                        }
                    } else {
                        self.programs.insert(program.name.clone(), program);
                    }
                }
            }
        }
        assert_eq!(self.programs.len(), 1);

        if let Some((_, program)) = self.programs.drain().next() {
            return Ok(program);
        }

        Err(Error::Unsolved)
    }
}

impl Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let program: Program = line.parse()?;
            self.programs.insert(program.name.clone(), program);
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let root = self.get_root()?;
        Ok(root.name.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut root = self.get_root()?;
        Ok(root.fix_weight(0).into())
    }
}
