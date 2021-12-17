use std::collections::BTreeMap;

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
struct Rules {
    people: Vec<String>,
    rules: BTreeMap<(usize, usize), isize>,
}

impl Rules {
    fn add_person(&mut self, name: String) -> usize {
        for i in 0..self.people.len() {
            if self.people[i] == name {
                return i;
            }
        }
        self.people.push(name);
        return self.people.len() - 1;
    }
}

#[derive(Debug)]
struct Table {
    people: Vec<String>,
    shuffle: usize,
    shuffles: Vec<Vec<usize>>,
}

impl Table {
    fn new(people: &Vec<String>) -> Self {
        let mut table = Table {
            people: people.clone(),
            shuffle: 0,
            shuffles: Vec::new(),
        };

        table.generate_shuffles();

        table
    }

    fn generate_shuffles(&mut self) {
        fn step(table: &mut Table, shuffle: Vec<usize>) {
            if shuffle.len() == table.people.len() {
                table.shuffles.push(shuffle);
                return;
            }

            for i in 0..table.people.len() {
                if !shuffle.contains(&i) {
                    let mut next = shuffle.clone();
                    next.push(i);
                    step(table, next);
                }
            }
        }

        let start = vec![0];
        step(self, start);
    }

    fn happiness(&self, rules: &Rules) -> isize {
        let mut happiness = 0isize;

        for i in 0..self.people.len() {
            let next_i = if i == self.people.len() - 1 { 0 } else { i + 1 };
            if let Some(delta) = rules.rules.get(&(
                self.shuffles[self.shuffle][i],
                self.shuffles[self.shuffle][next_i],
            )) {
                happiness += delta;
            }
            if let Some(delta) = rules.rules.get(&(
                self.shuffles[self.shuffle][next_i],
                self.shuffles[self.shuffle][i],
            )) {
                happiness += delta;
            }
        }

        happiness
    }

    fn shuffle(&mut self) -> bool {
        if self.shuffle == self.shuffles.len() - 1 {
            return false;
        }
        self.shuffle += 1;
        true
    }
}

fn load_input(filename: &str) -> Result<Rules, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut rules = Rules {
        people: Vec::new(),
        rules: BTreeMap::new(),
    };

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let line = line.replace(".", "");
        let line = line.replace(" would ", " ");
        let line = line.replace(" happiness units by sitting next to ", " ");

        let parts: Vec<&str> = line.split(" ").collect();
        let mut gain: isize = parts[2].parse().map_err(|e| Error::NAN(e))?;
        if parts[1] == "lose" {
            gain = -gain;
        }
        let a = rules.add_person(parts[0].to_string());
        let b = rules.add_person(parts[3].to_string());

        rules.rules.insert((a, b), gain);
    }

    Ok(rules)
}

fn main() -> Result<(), Error> {
    let rules = load_input(INPUT_FILE)?;

    println!("rules: {:?}", rules);

    let mut iters = 1usize;
    for i in 1..rules.people.len() {
        iters *= i;
    }

    println!("iters: {}", iters);

    let mut table = Table::new(&rules.people);
    println!(
        "table: {:?}  happiness:{}",
        table.shuffles[table.shuffle],
        table.happiness(&rules)
    );
    let mut max_happiness = table.happiness(&rules);

    while table.shuffle() {
        let happiness = table.happiness(&rules);
        if happiness > max_happiness {
            max_happiness = happiness;
        }

        println!(
            "table: {:?}  happiness:{}",
            table.shuffles[table.shuffle],
            table.happiness(&rules)
        );
    }

    println!("Max: {}", max_happiness);
    Ok(())
}
