#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<Vec<String>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut ids = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        ids.push(line.to_string());
    }

    Ok(ids)
}

fn main() -> Result<(), Error> {
    let ids = load_input(INPUT_FILE)?;

    let mut total_2 = 0usize;
    let mut total_3 = 0usize;

    for id in &ids {
        let mut counts: BTreeMap<char, usize> = BTreeMap::new();
        for c in id.chars() {
            let num = *counts.get(&c).unwrap_or(&0) + 1;
            counts.insert(c, num);
        }

        let mut has_2 = false;
        let mut has_3 = false;
        for (_, count) in &counts {
            match count {
                2 => has_2 = true,
                3 => has_3 = true,
                _ => {}
            }
        }

        if has_2 {
            total_2 += 1;
        }
        if has_3 {
            total_3 += 1;
        }
    }

    println!("{} * {} = {}", total_2, total_3, total_2 * total_3);

    Ok(())
}
