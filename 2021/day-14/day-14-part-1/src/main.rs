use std::collections::BTreeMap;

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<(Vec<char>, BTreeMap<(char, char), char>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut template: Vec<char> = Vec::new();
    let mut rules = BTreeMap::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if line.contains(" -> ") {
            let chars = line.chars().collect::<Vec<char>>();
            let a = chars[0];
            let b = chars[1];
            let c = chars[6];

            rules.insert((a, b), c);
        } else {
            template.extend_from_slice(&line.chars().collect::<Vec<char>>());
        }
    }

    Ok((template, rules))
}

fn main() -> Result<(), Error> {
    let (template, rules) = load_input(INPUT_FILE)?;

    println!("template: {:?}", template);
    if cfg!(debug_assertions) {
        println!("rules: {:?}", rules);
    }

    let mut cur = template.clone();
    for _ in 0..10 {
        let mut new = Vec::new();
        new.push(cur[0]);
        for i in 0..cur.len() - 1 {
            let insert = rules.get(&(cur[i], cur[i + 1])).unwrap();
            new.push(*insert);
            new.push(cur[i + 1]);
        }
        println!("len is {}", new.len());
        cur = new;
    }

    let mut letters = cur.clone();
    letters.sort();
    letters.dedup();

    println!("letters: {:?}", letters);

    let mut max = 0usize;
    let mut min = !0usize;
    for c in letters {
        let mut a = cur.clone();
        a.retain(|c1| *c1 == c);
        println!("{}: {}", c, a.len());
        if a.len() < min {
            min = a.len();
        }
        if a.len() > max {
            max = a.len();
        }
    }

    println!("max:{} min:{}  {}", max, min, max - min);

    Ok(())
}
