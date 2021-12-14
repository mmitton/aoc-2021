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

    let mut pairs = BTreeMap::new();
    for i in 0..template.len() - 1 {
        let key = (template[i], template[i + 1]);
        let num = *pairs.get(&key).unwrap_or(&0) + 1;
        pairs.insert(key, num);
    }

    for _ in 0..40 {
        let mut new_pairs = BTreeMap::new();
        for (pair, num) in pairs.iter() {
            let c = *rules.get(pair).unwrap();
            for key in [(pair.0, c), (c, pair.1)] {
                let num = *new_pairs.get(&key).unwrap_or(&0) + num;
                new_pairs.insert(key, num);
            }
        }

        pairs = new_pairs;
    }

    let mut letters: BTreeMap<char, usize> = BTreeMap::new();
    for (pair, num) in pairs.iter() {
        for c in [pair.0, pair.1] {
            let num = *letters.get(&c).unwrap_or(&0) + num;
            letters.insert(c, num);
        }
    }

    println!("{:?}", pairs);
    println!("{:?}", letters);

    let mut min = !0usize;
    let mut max = 0usize;
    for (c, num) in letters.iter_mut() {
        *num /= 2;
        if *c == template[0] || *c == template[template.len() - 1] {
            // println!("Found Start: {}", c);
            *num += 1;
        }
        println!("{}: {}", c, num);

        if *num < min {
            min = *num;
        }
        if *num > max {
            max = *num;
        }
    }

    println!("max:{} min:{}  {}", max, min, max - min);

    Ok(())
}
