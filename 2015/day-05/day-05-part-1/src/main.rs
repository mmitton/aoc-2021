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

    let mut inputs = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        inputs.push(line.to_string());
    }

    Ok(inputs)
}

fn is_nice(s: &str) -> bool {
    for bad in &["ab", "cd", "pq", "xy"] {
        if s.contains(bad) {
            return false;
        }
    }

    let chars: Vec<char> = s.chars().collect();
    let mut vowels = 0usize;
    let mut last_c = 0 as char;
    let mut has_double = false;
    for c in &chars {
        if last_c == *c {
            has_double = true;
        }
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
            _ => {}
        }

        last_c = *c;
    }

    return vowels >= 3 && has_double;
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    let mut total_nice = 0usize;
    for input in &inputs {
        let is_nice = is_nice(&input);
        if is_nice {
            total_nice += 1;
        }
        println!("{}: is_nice:{}", input, is_nice);
    }

    println!("Total Nice: {}", total_nice);
    Ok(())
}
