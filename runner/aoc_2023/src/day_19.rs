#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut, Range},
    str::FromStr,
};

#[derive(Copy, Clone, Debug)]
enum Var {
    X,
    M,
    A,
    S,
}

impl FromStr for Var {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(Error::InvalidInput(format!("Invalid Var: '{s}'"))),
        }
    }
}

#[derive(Debug)]
struct Part([usize; 4]);

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('{') else {
            return Err(Error::InvalidInput(format!("Part: '{s}'")));
        };
        let Some(s) = s.strip_suffix('}') else {
            return Err(Error::InvalidInput(format!("Part: '{s}'")));
        };

        let mut part = Part([0; 4]);

        for var in s.split(',') {
            let Some((var, num)) = var.split_once('=') else {
                return Err(Error::InvalidInput(format!("Part value: '{var}'")));
            };
            let var: Var = var.parse()?;
            part[var as usize] = num.parse()?;
        }

        Ok(part)
    }
}

impl Deref for Part {
    type Target = [usize; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Part {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    result: RuleResult,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (condition, result) = if let Some((condition, result)) = s.split_once(':') {
            (condition.parse()?, result.parse()?)
        } else {
            (Condition::All, s.parse()?)
        };

        Ok(Self { condition, result })
    }
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        match self.condition {
            Condition::All => true,
            Condition::LessThan(var, num) => part[var as usize] < num,
            Condition::GreaterThan(var, num) => part[var as usize] > num,
        }
    }
}

#[derive(Debug)]
enum Condition {
    LessThan(Var, usize),
    GreaterThan(Var, usize),
    All,
}

impl FromStr for Condition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((var, num)) = s.split_once('<') {
            let var = var.parse()?;
            let num = num.parse()?;
            return Ok(Condition::LessThan(var, num));
        }
        if let Some((var, num)) = s.split_once('>') {
            let var = var.parse()?;
            let num = num.parse()?;
            return Ok(Condition::GreaterThan(var, num));
        }
        Err(Error::InvalidInput(format!("Condition: '{s}'")))
    }
}

#[derive(Clone, Debug)]
enum RuleResult {
    SendTo(String),
    Accepted,
    Rejected,
}

impl FromStr for RuleResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RuleResult::Accepted),
            "R" => Ok(RuleResult::Rejected),
            _ => Ok(RuleResult::SendTo(s.into())),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_suffix('}') else {
            return Err(Error::InvalidInput(format!("Workflow: {s}")));
        };

        let Some((name, rules_str)) = s.split_once('{') else {
            return Err(Error::InvalidInput(format!("Workflow: {s}")));
        };

        let mut rules = Vec::new();
        for rule in rules_str.split(',') {
            rules.push(rule.parse()?);
        }

        Ok(Self {
            name: name.into(),
            rules,
        })
    }
}

impl Workflow {
    fn get_result(&self, part: &Part) -> Result<RuleResult, Error> {
        for rule in &self.rules {
            if rule.matches(part) {
                return Ok(rule.result.clone());
            }
        }

        Err(Error::Runner(format!(
            "Cannot match anything.  Workflow: {self:?}  Part: {part:?}"
        )))
    }

    fn add_constraints(&self, constraint: &Constraint) -> Vec<(Option<String>, Constraint)> {
        let mut constraint: Constraint = constraint.clone();
        let mut ret = Vec::new();
        for rule in &self.rules {
            match rule.condition {
                Condition::LessThan(var, num) => {
                    let range = &mut constraint[var as usize];
                    if range.start < num {
                        if range.end < num {
                            // Consume whole range
                            ret.push((rule.result.clone(), constraint));
                            break;
                        } else {
                            // Consume part of the range
                            let new_range = num..range.end;
                            range.end = num;
                            let mut new_constraint = constraint.clone();
                            new_constraint[var as usize] = new_range;
                            ret.push((rule.result.clone(), constraint));
                            constraint = new_constraint;
                        }
                    }
                }
                Condition::GreaterThan(var, num) => {
                    let range = &mut constraint[var as usize];
                    if range.end > num {
                        if range.start > num {
                            // Consume whole range
                            ret.push((rule.result.clone(), constraint));
                            break;
                        } else {
                            // Consume part of the range
                            let new_range = range.start..num + 1;
                            range.start = num + 1;
                            let mut new_constraint = constraint.clone();
                            new_constraint[var as usize] = new_range;
                            ret.push((rule.result.clone(), constraint));
                            constraint = new_constraint;
                        }
                    }
                }
                Condition::All => {
                    ret.push((rule.result.clone(), constraint));
                    break;
                }
            }
        }

        ret.iter()
            .filter_map(|(result, constraint)| match result {
                RuleResult::Accepted => Some((None, constraint.clone())),
                RuleResult::SendTo(name) => Some((Some(name.clone()), constraint.clone())),
                RuleResult::Rejected => None,
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Constraint([Range<usize>; 4]);

impl Default for Constraint {
    fn default() -> Self {
        Self([1..4001, 1..4001, 1..4001, 1..4001])
    }
}

impl Deref for Constraint {
    type Target = [Range<usize>; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Constraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Day19 {
    workflows: BTreeMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Day19 {
    pub fn new() -> Self {
        Self {
            workflows: BTreeMap::new(),
            parts: Vec::new(),
        }
    }
}

impl Runner for Day19 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let workflow: Workflow = line.parse()?;
            self.workflows.insert(workflow.name.clone(), workflow);
        }

        for line in lines {
            self.parts.push(line.parse()?);
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;
        for part in self.parts.iter() {
            let mut workflow_name: String = "in".into();
            loop {
                let Some(workflow) = self.workflows.get(&workflow_name) else {
                    return Err(Error::Runner(format!(
                        "Cannot find workflow '{workflow_name}'"
                    )));
                };
                match workflow.get_result(part)? {
                    RuleResult::Accepted => {
                        ans += part.iter().sum::<usize>();
                        println!("Accepted {part:?}");
                        break;
                    }
                    RuleResult::Rejected => {
                        println!("Rejected {part:?}");
                        break;
                    }
                    RuleResult::SendTo(name) => workflow_name.clone_from(&name),
                }
            }
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;
        let mut work: Vec<(String, Constraint)> = Vec::new();
        work.push(("in".to_string(), Constraint::default()));
        while let Some((workflow_name, constraint)) = work.pop() {
            let Some(workflow) = self.workflows.get(&workflow_name) else {
                return Err(Error::Runner(format!(
                    "Cannot find workflow '{workflow_name}'"
                )));
            };

            for (workflow_name, constraint) in workflow.add_constraints(&constraint) {
                if let Some(workflow_name) = workflow_name {
                    work.push((workflow_name, constraint));
                } else {
                    println!("Accepted {constraint:?}");
                    ans += constraint
                        .iter()
                        .map(|range| range.end - range.start)
                        .product::<usize>();
                }
            }
        }
        Ok(ans.into())
    }
}
