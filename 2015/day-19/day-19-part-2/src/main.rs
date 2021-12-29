#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use rand::{thread_rng, Rng};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::NAN(e)
    }
}

#[derive(Debug)]
struct Rule {
    from: String,
    to: String,
}

fn load_input(filename: &str) -> Result<(Vec<Rule>, Vec<String>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut rules: Vec<Rule> = Vec::new();
    let mut molecules: Vec<String> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if line.contains(" => ") {
            let parts: Vec<&str> = line.split(" => ").collect();
            let rule = Rule {
                from: parts[0].to_string(),
                to: parts[1].to_string(),
            };
            rules.push(rule);
        } else {
            molecules.push(line.to_string());
        }
    }

    rules.sort_by(|a, b| b.to.len().cmp(&a.to.len()));
    Ok((rules, molecules))
}

fn randomize_rules(rules: &mut Vec<Rule>) {
    let mut rng = thread_rng();
    rng.shuffle(rules);
}

fn main() -> Result<(), Error> {
    let (mut rules, molecules) = load_input(INPUT_FILE)?;

    for molecule in &molecules {
        let mut steps;
        'rules_loop: loop {
            let mut molecule = molecule.clone();
            randomize_rules(&mut rules);
            steps = 0;
            'search_loop: loop {
                let mut found = false;
                for rule in &rules {
                    let rule_len = rule.to.len();
                    if rule_len > molecule.len() {
                        break;
                    }
                    let mut i = 0;
                    loop {
                        if molecule[i..i + rule_len] == rule.to {
                            let mut output = molecule[0..i].to_string();
                            output.push_str(&rule.from);
                            output.push_str(&molecule[i + rule_len..]);
                            molecule = output;
                            steps += 1;
                            found = true;
                        } else {
                            i += 1;
                        }
                        if i + rule_len > molecule.len() {
                            break;
                        }
                    }
                }

                if found {
                    continue 'search_loop;
                }

                if &molecule == "e" {
                    break 'rules_loop;
                }

                continue 'rules_loop;
            }
        }

        println!("molecule: '{}'  steps: {}", molecule, steps);
    }

    Ok(())
}
