#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug, Clone)]
enum Rule {
    Const(String),
    Inner(usize),
    And(Vec<Rule>),
    Or(Vec<Rule>),
}

pub struct Day19 {
    rules: HashMap<usize, Rule>,
    inputs: HashSet<String>,
}

impl Day19 {
    pub fn new() -> Self {
        Self {
            rules: HashMap::default(),
            inputs: HashSet::default(),
        }
    }

    fn process_rule(&self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::Const(s) => vec![s.clone()],
            Rule::And(group) => {
                let mut valid = Vec::new();
                for r in group {
                    let sub = self.process_rule(r);
                    if valid.is_empty() {
                        valid.extend_from_slice(&sub);
                    } else {
                        for i in (0..valid.len()).rev() {
                            for sub in &sub {
                                let mut new = valid[i].clone();
                                new.push_str(sub);
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
                    let sub = self.process_rule(group);
                    valid.extend_from_slice(&sub);
                }

                valid
            }
            Rule::Inner(r) => self.buildout_rule(*r),
        }
    }

    fn buildout_rule(&self, rule: usize) -> Vec<String> {
        let rule = self.rules.get(&rule).unwrap();
        self.process_rule(rule)
    }

    fn process<F>(&self, f: F) -> usize
    where
        F: Fn(usize, usize) -> bool,
    {
        let patterns_42 = self.buildout_rule(42);
        let patterns_31 = self.buildout_rule(31);

        let mut num_valid = 0;
        for orig_input in self.inputs.iter() {
            let mut input = orig_input.as_str();
            let mut num_42 = 0;
            let mut num_31 = 0;

            loop {
                let mut found = false;
                for pattern in &patterns_42 {
                    if input.starts_with(pattern) {
                        found = true;
                        input = &input[pattern.len()..];
                        num_42 += 1;
                    }
                }
                if !found {
                    break;
                }
            }

            loop {
                let mut found = false;
                for pattern in &patterns_31 {
                    if input.starts_with(pattern) {
                        found = true;
                        input = &input[pattern.len()..];
                        num_31 += 1;
                    }
                }
                if !found {
                    break;
                }
            }
            if !input.is_empty() {
                continue;
            }
            if !f(num_42, num_31) {
                continue;
            }

            num_valid += 1;
        }
        num_valid
    }
}

impl Runner for Day19 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
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
                            group.push(Rule::Inner(num.parse()?));
                        }
                        groups.push(Rule::And(group));
                    }
                    self.rules.insert(rule_num, Rule::Or(groups));
                } else {
                    let mut group = Vec::new();
                    for num in parts[1].split(' ') {
                        group.push(Rule::Inner(num.parse()?));
                    }
                    self.rules.insert(rule_num, Rule::And(group));
                }
            } else {
                self.inputs.insert(line.to_string());
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
        if self.inputs.len() < 100 {
            let valid_messages = self.buildout_rule(0);
            println!("{}", valid_messages.len());
            Ok(valid_messages
                .iter()
                .filter(|m| self.inputs.contains(m.as_str()))
                .count()
                .into())
        } else {
            Ok(self.process(|num42, num31| num42 == 2 && num31 == 1).into())
        }
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .process(|num42, num31| {
                num42 != 0 && num31 != 0 && (num42 as isize - num31 as isize) >= 1
            })
            .into())
    }
}
