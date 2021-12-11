#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    NAN(std::num::ParseIntError),
    NoSolution,
}

#[derive(Debug)]
struct Bag {
    name: String,
    contains: Vec<(usize, String)>,
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

fn count_inside(bags: &Vec<Bag>, i: usize) -> usize {
    let mut inside = 0usize;

    for j in 0..bags[i].contains.len() {
        println!(
            "{} holds {} {}",
            bags[i].name, bags[i].contains[j].0, bags[i].contains[j].1
        );
        inside += bags[i].contains[j].0;

        // Look for .1 and loop

        for k in 0..bags.len() {
            if bags[k].name == bags[i].contains[j].1 {
                inside += bags[i].contains[j].0 * count_inside(bags, k);
            }
        }
    }

    inside
}

fn main() -> Result<(), Error> {
    let bags = load_input(INPUT_FILE)?;

    for i in 0..bags.len() {
        if bags[i].name == "shiny gold" {
            println!("shiny bag holds: {}", count_inside(&bags, i));
            return Ok(());
        }
    }

    return Err(Error::NoSolution);
}
