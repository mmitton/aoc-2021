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

#[derive(Debug)]
struct Program {
    name: String,
    weight: isize,
    holding: Vec<Program>,
    holding_names: Vec<String>,
}

impl Program {
    fn weight(&self) -> isize {
        let mut weight = self.weight;
        for h in &self.holding {
            weight += h.weight();
        }

        weight
    }

    fn weights(&self) -> BTreeMap<isize, usize> {
        let mut weights = BTreeMap::new();
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

fn load_input(filename: &str) -> Result<Vec<Program>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut programs: Vec<Program> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(" -> ").collect();
        let name_weight = parts[0].replace(" (", " ");
        let name_weight = name_weight.replace(")", "");
        let name_weight: Vec<&str> = name_weight.split(" ").collect();
        let mut program = Program {
            name: name_weight[0].to_string(),
            weight: name_weight[1].parse().map_err(|e| Error::NAN(e))?,
            holding: Vec::new(),
            holding_names: Vec::new(),
        };

        if parts.len() == 2 {
            for holding in parts[1].split(", ") {
                program.holding_names.push(holding.to_string());
            }
        }

        programs.push(program);
    }

    Ok(programs)
}

fn main() -> Result<(), Error> {
    let mut programs = load_input(INPUT_FILE)?;
    let mut merged = true;
    while merged {
        merged = false;

        'find_loop: for i in 0..programs.len() {
            if programs[i].holding.len() != programs[i].holding_names.len() {
                for j in (0..programs.len()).rev() {
                    if i == j {
                        continue;
                    }
                    if programs[i].holding_names.contains(&programs[j].name) {
                        if programs[j].holding.len() == programs[j].holding_names.len() {
                            let mut i = i;
                            if i > j {
                                i -= 1;
                            }

                            let sub_program = programs.remove(j);
                            programs[i].holding.push(sub_program);
                            merged = true;
                            break 'find_loop;
                        }
                    }
                }
            }
        }
    }

    assert!(programs.len() == 1);
    let mut program = programs.remove(0);
    println!("{}", program.name);

    println!("Answer: {}", program.fix_weight(0));
    for i in 0..program.holding.len() {
        println!(
            "{}  {} ({})",
            i,
            program.holding[i].name,
            program.holding[i].weight()
        );
    }

    Ok(())
}
