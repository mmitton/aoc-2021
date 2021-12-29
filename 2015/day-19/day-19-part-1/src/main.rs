#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

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

    Ok((rules, molecules))
}

fn main() -> Result<(), Error> {
    let (rules, molecules) = load_input(INPUT_FILE)?;

    for molecule in &molecules {
        let mut outputs = Vec::new();
        for rule in &rules {
            let rule_len = rule.from.len();
            for i in 0..molecule.len() - rule_len + 1 {
                if molecule[i..i + rule_len] == rule.from {
                    let mut output = molecule[0..i].to_string();
                    output.push_str(&rule.to);
                    output.push_str(&molecule[i + rule_len..]);
                    if !outputs.contains(&output) {
                        outputs.push(output);
                    }
                }
            }
        }

        if cfg!(debug_assertions) {
            println!(
                "Molecule => '{}' makes {} molecules",
                molecule,
                outputs.len()
            );
            for output in &outputs {
                println!("      => '{}'", output);
            }
        } else {
            println!("Answer: {}", outputs.len());
        }
    }

    Ok(())
}
