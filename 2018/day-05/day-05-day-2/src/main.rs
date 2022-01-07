#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<Vec<String>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut polymers = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        polymers.push(line);
    }

    Ok(polymers)
}

fn react(polymer: &str) -> String {
    let chars: Vec<char> = polymer.chars().collect();
    let mut ll = vec![0; chars.len()];
    for i in 0..chars.len() {
        if i != chars.len() - 1 {
            ll[i] = i + 1;
        } else {
            ll[i] = 0;
        }
    }
    let mut head = 0;

    loop {
        let mut changed = false;

        let mut idx = head;
        let mut last = None;
        loop {
            if ll[idx] == 0 {
                break;
            }

            let next_idx = ll[idx];

            let same_char = chars[idx].to_uppercase().eq(chars[next_idx].to_uppercase());
            let trigger =
                if same_char && chars[idx].is_lowercase() && chars[next_idx].is_uppercase() {
                    true
                } else if same_char && chars[idx].is_uppercase() && chars[next_idx].is_lowercase() {
                    true
                } else {
                    false
                };

            if trigger {
                changed = true;
                match last {
                    None => {
                        // at head, just move forward two
                        head = ll[next_idx];
                        idx = head;
                    }
                    Some(last) => {
                        // set last link to two ahead
                        ll[last] = ll[next_idx];
                        idx = ll[last];
                        if idx == 0 {
                            break;
                        }
                    }
                }
            } else {
                last = Some(idx);
                idx = next_idx;
            }
        }

        if !changed {
            break;
        }
    }

    let mut new_polymer = String::with_capacity(polymer.len());
    let mut idx = head;
    loop {
        new_polymer.push(chars[idx]);

        if ll[idx] == 0 {
            break;
        }
        idx = ll[idx];
    }

    new_polymer
}

fn main() -> Result<(), Error> {
    let polymers = load_input(INPUT_FILE)?;

    for polymer in &polymers {
        let mut best = usize::MAX;

        for c in 'a'..='z' {
            let mut trimmed_polymer = String::with_capacity(polymer.len());
            for pc in polymer.chars() {
                if pc != c && pc != c.to_uppercase().nth(0).unwrap() {
                    trimmed_polymer.push(pc);
                }
            }

            let new_polymer = react(&trimmed_polymer);
            if new_polymer.len() < best {
                if cfg!(debug_assertions) {
                    println!("{} => {}", trimmed_polymer, new_polymer);
                }
                best = new_polymer.len();
            }
        }

        println!("{} units", best);
    }

    Ok(())
}
