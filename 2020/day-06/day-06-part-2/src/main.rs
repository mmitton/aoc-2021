#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

type Answers = [bool; 26];

#[derive(Debug)]
struct Group {
    people: Vec<Answers>,
}

impl Group {
    fn num_responses(&self) -> usize {
        let mut responses = 0usize;

        for i in 0..26 {
            let mut num = 0usize;
            for person in &self.people {
                if person[i] {
                    num += 1;
                }
            }

            if num == self.people.len() {
                responses += 1;
            }
        }

        responses
    }
}

fn load_input(filename: &str) -> Result<Vec<Group>, Error> {
    let mut groups: Vec<Group> = Vec::new();

    let file = File::open(filename).map_err(|e| Error::IO(e))?;
    let mut group = Group { people: Vec::new() };
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            if group.people.len() > 0 {
                groups.push(group);
            }
            group = Group { people: Vec::new() };
            continue;
        }

        let mut person = [false; 26];
        for c in line.chars() {
            let i = c as u8 - 'a' as u8;
            person[i as usize] = true;
        }
        group.people.push(person);
    }
    if group.people.len() > 0 {
        groups.push(group);
    }

    Ok(groups)
}

fn main() -> Result<(), Error> {
    let groups = load_input(INPUT_FILE)?;

    let mut answer = 0usize;
    for group in &groups {
        answer += group.num_responses();
    }

    println!("Answer: {}", answer);
    return Ok(());
}
