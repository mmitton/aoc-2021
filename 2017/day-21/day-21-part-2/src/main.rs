#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
}

#[derive(Debug)]
struct Rule {
    output: Vec<Vec<bool>>,
    inputs: Vec<Vec<Vec<bool>>>,
}

fn print_image(image: &Vec<Vec<bool>>) {
    println!("IMAGE:");
    let mut on = 0;
    for row in image {
        print!("  ");
        for c in row {
            print!("{}", if *c { "#" } else { "." });
            on += if *c { 1 } else { 0 };
        }
        println!();
    }
    println!("Total On: {}", on);
    println!();
}

impl std::fmt::Display for Rule {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn write_row(row: &Vec<bool>, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            for c in row {
                write!(fmt, "{}", if *c { "#" } else { "." })?;
            }

            Ok(())
        }
        for y in 0..self.output.len() {
            if y == 0 {
                write!(fmt, "  Output:  ")?;
            } else {
                write!(fmt, "           ")?;
            }
            write!(fmt, "  ")?;
            write_row(&self.output[y], fmt)?;

            if y != self.inputs[0].len() {
                if y == 0 {
                    write!(fmt, "  Inputs:  ")?;
                } else {
                    write!(fmt, "           ")?;
                }

                for i in 0..self.inputs.len() {
                    write!(fmt, "  ")?;
                    write_row(&self.inputs[i][y], fmt)?;
                }
            }

            writeln!(fmt)?;
        }

        Ok(())
    }
}

impl TryFrom<&str> for Rule {
    type Error = Error;

    fn try_from(s: &str) -> Result<Rule, Error> {
        let parts: Vec<&str> = s.split(" => ").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidInput(s.to_string()));
        }

        let mut rule = Rule {
            output: Vec::new(),
            inputs: Vec::new(),
        };

        for line in parts[1].split("/") {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c == '#');
            }
            rule.output.push(row);
        }

        let mut input = Vec::new();
        for line in parts[0].split("/") {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c == '#');
            }
            input.push(row);
        }

        rule.inputs.push(input.clone());

        let max = input.len() - 1;
        for i in 1..=3 {
            let mut rotated = Vec::new();
            for y in 0..input.len() {
                let mut row = Vec::new();
                for x in 0..input.len() {
                    match i {
                        1 => row.push(input[max - x][y]),
                        2 => row.push(input[max - y][max - x]),
                        3 => row.push(input[x][max - y]),
                        _ => unreachable!(),
                    }
                }
                rotated.push(row);
            }

            // Make flips
            for i in 0..2 {
                let mut flipped = Vec::new();
                for y in 0..rotated.len() {
                    let mut row = Vec::new();
                    for x in 0..rotated.len() {
                        match i {
                            0 => row.push(rotated[y][max - x]),
                            1 => row.push(rotated[max - y][x]),
                            _ => unreachable!(),
                        }
                    }
                    flipped.push(row);
                }
                if !rule.inputs.contains(&flipped) {
                    rule.inputs.push(flipped);
                }
            }

            if !rule.inputs.contains(&rotated) {
                rule.inputs.push(rotated);
            }
        }

        Ok(rule)
    }
}

fn load_input(filename: &str) -> Result<Vec<Rule>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut rules: Vec<Rule> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        rules.push(line.try_into()?);
    }

    Ok(rules)
}

fn mutate_image(image: &Vec<Vec<bool>>, rules: &Vec<Rule>) -> Vec<Vec<bool>> {
    let split = if image.len() % 2 == 0 { 2 } else { 3 };
    let splits = image.len() / split;

    let mut output = Vec::new();
    for _ in 0..image.len() + splits {
        output.push(vec![false; image.len() + splits]);
    }

    let mut output_y = 0usize;
    for y in (0..image.len()).step_by(split) {
        let mut output_x = 0usize;
        for x in (0..image.len()).step_by(split) {
            let mut found_rule = false;
            'rule_loop: for rule in rules {
                if rule.inputs[0].len() != split {
                    continue;
                }
                for input in &rule.inputs {
                    let mut matches = true;
                    'match_loop: for y1 in 0..split {
                        for x1 in 0..split {
                            if input[y1][x1] != image[y + y1][x + x1] {
                                matches = false;
                                break 'match_loop;
                            }
                        }
                    }

                    if matches {
                        // Copy output of rule to output image and break out of loop
                        for y1 in 0..split + 1 {
                            for x1 in 0..split + 1 {
                                output[output_y + y1][output_x + x1] = rule.output[y1][x1];
                            }
                        }
                        found_rule = true;
                        break 'rule_loop;
                    }
                }
            }
            assert!(found_rule);
            output_x += split + 1;
        }
        output_y += split + 1;
    }

    output
}

fn main() -> Result<(), Error> {
    let rules = load_input(INPUT_FILE)?;

    let iters = if rules.len() == 2 { 2 } else { 18 };

    println!("iters: {}", iters);
    if true || cfg!(debug_assertions) {
        for rule in &rules {
            println!("{}", rule);
        }
    }

    let mut image = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    print_image(&image);
    for _ in 0..iters {
        image = mutate_image(&image, &rules);
        print_image(&image);
    }

    Ok(())
}
