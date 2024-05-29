#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug, Clone)]
enum Rule {
    Const(String),
    Rule(usize),
    And(Vec<Rule>),
    Or(Vec<Rule>),
}

pub struct Day19 {
    rules: HashMap<usize, Rule>,
    inputs: Vec<String>,
}

impl Day19 {
    pub fn new() -> Self {
        Self {
            rules: HashMap::default(),
            inputs: Vec::new(),
        }
    }

    fn process_rule(&self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::Const(s) => vec![s.clone()],
            Rule::And(group) => {
                let mut valid = Vec::new();
                for r in group {
                    let sub = self.process_rule(r);
                    if valid.len() == 0 {
                        valid.extend_from_slice(&sub);
                    } else {
                        for i in (0..valid.len()).rev() {
                            for sub in &sub {
                                let mut new = valid[i].clone();
                                new.push_str(&sub);
                                valid.push(new);
                            }
                            valid.remove(i);
                        }
                    }
                }

                valid
            }
            Rule::Or(groups) => {
                let mut valid = Vec::new();

                for group in groups.iter() {
                    let sub = process_rule(group);
                    valid.extend_from_slice(&sub);
                }

                valid
            }
            Rule::Rule(r) => self.buildout_rule(r),
        }
    }

    fn buildout_rule(&self, rule: usize) -> Vec<String> {
        let rule = rules.get(&rule).unwrap();
        self.process_rule(rule)
    }
}

impl Runner for Day19 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for line in lines.iter() {
            if line.contains(':') {
                let parts: Vec<&str> = line.split(": ").collect();
                let rule_num: usize = parts[0].parse()?;
                if parts[1].starts_with('\"') {
                    self.rules.insert(
                        rule_num,
                        Rule::Const(parts[1].replace('\"', "").to_string()),
                    );
                } else if parts[1].contains('|') {
                    let mut groups = Vec::new();
                    for part in parts[1].split(" | ") {
                        let mut group = Vec::new();
                        for num in part.split(' ') {
                            group.push(Rule::Rule(num.parse()?));
                        }
                        groups.push(Rule::And(group));
                    }
                    self.rules.insert(rule_num, Rule::Or(groups));
                } else {
                    let mut group = Vec::new();
                    for num in parts[1].split(' ') {
                        group.push(Rule::Rule(num.parse()?));
                    }
                    self.rules.insert(rule_num, Rule::And(group));
                }
            } else {
                self.inputs.push(line.to_string());
            }
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let valid_messages = self.buildout_rule(0);
        Ok(self
            .inputs
            .iter()
            .filter(|input| valid_messages.contains(input))
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }
}
