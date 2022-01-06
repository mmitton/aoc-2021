#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Const(String),
    Rule(usize),
    And(Vec<Rule>),
    Or(Vec<Rule>),
}

fn load_input(filename: &str) -> Result<(BTreeMap<usize, Rule>, Vec<String>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut rules = BTreeMap::new();
    let mut inputs = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        if line.contains(":") {
            let parts: Vec<&str> = line.split(": ").collect();
            let rule_num: usize = parts[0].parse()?;
            if parts[1].starts_with("\"") {
                rules.insert(
                    rule_num,
                    Rule::Const(parts[1].replace("\"", "").to_string()),
                );
            } else if parts[1].contains("|") {
                let mut groups = Vec::new();
                for part in parts[1].split(" | ") {
                    let mut group = Vec::new();
                    for num in part.split(" ") {
                        group.push(Rule::Rule(num.parse()?));
                    }
                    groups.push(Rule::And(group));
                }
                rules.insert(rule_num, Rule::Or(groups));
            } else {
                let mut group = Vec::new();
                for num in parts[1].split(" ") {
                    group.push(Rule::Rule(num.parse()?));
                }
                rules.insert(rule_num, Rule::And(group));
            }
        } else {
            inputs.push(line.to_string());
        }
    }

    Ok((rules, inputs))
}

fn buildout_rule(rule: usize, rules: &BTreeMap<usize, Rule>) -> Vec<String> {
    fn process_rule(rule: Rule, rules: &BTreeMap<usize, Rule>) -> Vec<String> {
        match rule {
            Rule::Const(s) => vec![s.clone()],
            Rule::And(group) => {
                let mut valid = Vec::new();
                for r in group {
                    let sub = process_rule(r, rules);
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

                for group in groups {
                    let sub = process_rule(group, rules);
                    valid.extend_from_slice(&sub);
                }

                valid
            }
            Rule::Rule(r) => buildout_rule(r, rules),
        }
    }

    let rule = rules.get(&rule).unwrap().clone();
    process_rule(rule, rules)
}

fn main() -> Result<(), Error> {
    let (rules, inputs) = load_input(INPUT_FILE)?;

    if cfg!(debug_assertions) {
        for (rule_num, rule) in &rules {
            println!("Rule {}: {:?}", rule_num, rule);
        }
    }

    let valid_messages = buildout_rule(0, &rules);
    println!("Valid Inputs: {}", valid_messages.len());

    let mut num_valid = 0;
    for input in &inputs {
        if valid_messages.contains(input) {
            num_valid += 1;
        }
    }
    println!("Number of valid messages: {}", num_valid);

    Ok(())
}
