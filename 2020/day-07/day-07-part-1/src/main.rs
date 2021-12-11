#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Bag {
    name: String,
    contains: Vec<(usize, String)>,
}

impl Bag {
    fn contains(&self, name: &str) -> bool {
        for i in 0..self.contains.len() {
            if &self.contains[i].1 == name {
                return true;
            }
        }

        false
    }
}

fn load_input(filename: &str) -> Result<Vec<Bag>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let mut bags: Vec<Bag> = Vec::new();

    let file = File::open(filename).map_err(|e| Error::IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let parts = line.split(" bags contain ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(Error::InvalidInput(line.to_string()));
        }

        let name = parts[0].to_string();
        let mut contains = Vec::new();
        if parts[1] != "no other bags." {
            for part in parts[1][..parts[1].len() - 1].split(", ") {
                let first_space = part.find(" ").unwrap();
                let last_space = part.rfind(" ").unwrap();
                let number = part[0..first_space]
                    .parse::<usize>()
                    .map_err(|e| Error::NAN(e))?;
                let bag = part[first_space + 1..last_space].to_string();
                contains.push((number, bag));
            }
        }

        bags.push(Bag {
            name: name,
            contains: contains,
        });
    }

    Ok(bags)
}

fn find_top(top_bags: &mut Vec<String>, bags: &Vec<Bag>, i: usize) {
    if !top_bags.contains(&bags[i].name) {
        top_bags.push(bags[i].name.clone());
    }

    println!("bag: {:?}", bags[i]);

    for j in 0..bags.len() {
        if j == i {
            continue;
        }
        if bags[j].contains(&bags[i].name) {
            find_top(top_bags, bags, j);
        }
    }
}

fn main() -> Result<(), Error> {
    let bags = load_input(INPUT_FILE)?;

    let mut top_bags = Vec::new();
    for i in 0..bags.len() {
        if bags[i].contains("shiny gold") {
            find_top(&mut top_bags, &bags, i);
        }
    }

    println!("top_bags: {:?}", top_bags);
    println!("Total: {}", top_bags.len());

    return Ok(());
}
