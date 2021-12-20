#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Program {
    name: String,
    weight: usize,
    holding: Vec<Program>,
    holding_names: Vec<String>,
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
    let program = programs.remove(0);
    println!("{}", program.name);

    Ok(())
}
