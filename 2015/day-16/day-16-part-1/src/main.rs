const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

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

#[derive(Clone, Debug)]
struct Sue {
    num: usize,
    known: BTreeMap<String, usize>,
}

impl Sue {
    fn matches(&self, results: &BTreeMap<String, usize>) -> bool {
        for (rkey, rval) in results.iter() {
            if let Some(val) = self.known.get(rkey) {
                if val != rval {
                    return false;
                }
            }
        }

        true
    }
}

impl TryFrom<&str> for Sue {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let split = s.find(": ").unwrap();
        let num: usize = (s[0..split].split(" ").collect::<Vec<&str>>())[1].parse()?;

        let mut known = BTreeMap::new();
        for part in s[split + 2..].split(", ") {
            let parts: Vec<&str> = part.split(": ").collect();
            known.insert(parts[0].to_string(), parts[1].parse()?);
        }

        Ok(Sue {
            num: num,
            known: known,
        })
    }
}

fn load_input(filename: &str) -> Result<Vec<Sue>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut sues: Vec<Sue> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        sues.push(line.try_into()?);
    }

    Ok(sues)
}

fn main() -> Result<(), Error> {
    let sues = load_input(INPUT_FILE)?;

    let mut results = BTreeMap::new();
    results.insert("children".to_string(), 3);
    results.insert("cats".to_string(), 7);
    results.insert("samoyeds".to_string(), 2);
    results.insert("pomeranians".to_string(), 3);
    results.insert("akitas".to_string(), 0);
    results.insert("vizslas".to_string(), 0);
    results.insert("goldfish".to_string(), 5);
    results.insert("trees".to_string(), 3);
    results.insert("cars".to_string(), 2);
    results.insert("perfumes".to_string(), 1);

    for sue in &sues {
        if sue.matches(&results) {
            println!("Sue: {:?}", sue);
        }
    }

    Ok(())
}
