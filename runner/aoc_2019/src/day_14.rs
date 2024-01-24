use std::{cmp::Ordering, collections::HashMap};

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Product {
    chem: usize,
    amount: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Rule {
    makes: Product,
    requires: Vec<Product>,
}

pub struct Day14 {
    rules: Vec<Rule>,
}

impl Day14 {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn make(&self, fuel: usize) -> Vec<usize> {
        let mut inventory = vec![0; self.rules.len()];
        let mut work = Vec::new();

        macro_rules! add_work {
            ($chem:expr, $amount:expr) => {{
                if $chem == 0 {
                    inventory[$chem] += $amount;
                } else if inventory[$chem] >= $amount {
                    inventory[$chem] -= $amount;
                } else {
                    let needed = $amount - inventory[$chem];
                    let mut units = needed / self.rules[$chem].makes.amount;
                    if needed % self.rules[$chem].makes.amount != 0 {
                        units += 1;
                    }

                    for requires in self.rules[$chem].requires.iter() {
                        work.push((requires.chem, requires.amount * units));
                    }

                    inventory[$chem] += units * self.rules[$chem].makes.amount;
                    inventory[$chem] -= $amount;
                }
            }};
        }

        add_work!(1, fuel);

        while let Some((chem, amount)) = work.pop() {
            add_work!(chem, amount);
        }

        inventory
    }
}

impl Runner for Day14 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let mut names: HashMap<&str, usize> = HashMap::new();
        fn map_name<'a>(names: &mut HashMap<&'a str, usize>, name: &'a str) -> usize {
            if let Some(&id) = names.get(name) {
                id
            } else {
                let id = names.len();
                names.insert(name, id);
                id
            }
        }
        map_name(&mut names, "ORE");
        map_name(&mut names, "FUEL");
        let mut rules = Vec::new();
        let lines = Lines::from_path(path, LinesOpt::REMOVE_EMPTY)?;
        for line in lines.iter() {
            let (requires, makes) = line.split_once(" => ").unwrap();
            let (makes_amount, makes_name) = makes.split_once(' ').unwrap();

            let mut rule = Rule {
                makes: Product {
                    chem: map_name(&mut names, makes_name),
                    amount: makes_amount.parse()?,
                },
                requires: Vec::new(),
            };

            for requires in requires.split(", ") {
                let (requires_amount, requires_name) = requires.split_once(' ').unwrap();
                rule.requires.push(Product {
                    chem: map_name(&mut names, requires_name),
                    amount: requires_amount.parse()?,
                });
            }

            rules.push(rule);
        }

        self.rules = vec![Rule::default(); names.len()];
        for rule in rules.iter() {
            assert_eq!(self.rules[rule.makes.chem], Rule::default());
            self.rules[rule.makes.chem] = rule.clone();
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let inventory = self.make(1);
        Ok(inventory[0].into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut fuel = 1;
        const MAX_ORE: usize = 1000000000000;
        loop {
            let inventory = self.make(fuel);
            if inventory[0] > MAX_ORE {
                break;
            }

            fuel *= 2;
        }
        let mut lower = fuel / 2;
        loop {
            let mid = (fuel + lower) / 2;
            let inventory = self.make(mid);
            match inventory[0].cmp(&MAX_ORE) {
                Ordering::Equal => {
                    fuel = mid;
                    break;
                }
                Ordering::Greater => {
                    fuel = mid;
                }
                Ordering::Less => {
                    if mid == lower {
                        fuel = mid;
                        break;
                    }
                    lower = mid;
                }
            }
        }
        Ok(fuel.into())
    }
}
