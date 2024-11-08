#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

struct Rule {
    output: Vec<Vec<bool>>,
    inputs: Vec<Vec<Vec<bool>>>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

#[derive(Default)]
pub struct Day21 {
    rules: Vec<Rule>,
}

struct Image {
    rows: Vec<Vec<bool>>,
}

impl Image {
    fn new() -> Self {
        Self {
            rows: vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, true, true],
            ],
        }
    }

    fn enhance(&mut self, rules: &[Rule]) {
        let split = if self.rows.len() % 2 == 0 { 2 } else { 3 };
        let splits = self.rows.len() / split;

        let mut new_rows = Vec::new();
        for _ in 0..self.rows.len() + splits {
            new_rows.push(vec![false; self.rows.len() + splits]);
        }

        let mut new_rows_y = 0usize;
        for y in (0..self.rows.len()).step_by(split) {
            let mut new_rows_x = 0usize;
            for x in (0..self.rows.len()).step_by(split) {
                let mut found_rule = false;
                'rule_loop: for rule in rules {
                    if rule.inputs[0].len() != split {
                        continue;
                    }
                    for input in &rule.inputs {
                        let mut matches = true;
                        'match_loop: for (y1, row) in input.iter().enumerate().take(split) {
                            for (x1, col) in row.iter().enumerate().take(split) {
                                if *col != self.rows[y + y1][x + x1] {
                                    matches = false;
                                    break 'match_loop;
                                }
                            }
                        }

                        if matches {
                            // Copy output of rule to new_rows image and break out of loop
                            for y1 in 0..split + 1 {
                                for x1 in 0..split + 1 {
                                    new_rows[new_rows_y + y1][new_rows_x + x1] =
                                        rule.output[y1][x1];
                                }
                            }
                            found_rule = true;
                            break 'rule_loop;
                        }
                    }
                }
                assert!(found_rule);
                new_rows_x += split + 1;
            }
            new_rows_y += split + 1;
        }

        self.rows = new_rows;
    }

    fn num_on(&self) -> usize {
        self.rows
            .iter()
            .map(|l| l.iter().filter(|v| **v).count())
            .sum()
    }
}

impl Day21 {
    pub fn new() -> Self {
        Self::default()
    }

    fn enhance(&self, iters: usize) -> usize {
        let mut image = Image::new();

        for _ in 0..iters {
            image.enhance(&self.rules);
        }

        image.num_on()
    }
}

impl Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.rules.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let iters = if self.rules.len() == 2 { 2 } else { 5 };
        Ok(self.enhance(iters).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let iters = if self.rules.len() == 2 { 2 } else { 18 };
        Ok(self.enhance(iters).into())
    }
}
