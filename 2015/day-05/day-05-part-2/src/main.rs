#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample-2.txt";
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
    let c: Vec<char> = s.chars().collect();

    let mut found_part_1 = false;
    'part_1: for i in 0..c.len() - 3 {
        for j in i + 2..c.len() - 1 {
            if c[i] == c[j] && c[i + 1] == c[j + 1] {
                found_part_1 = true;
                break 'part_1;
            }
        }
    }
    if !found_part_1 {
        return false;
    }

    let mut found_part_2 = false;
    for i in 0..c.len() - 2 {
        if c[i] == c[i + 2] {
            found_part_2 = true;
        }
    }
    if !found_part_2 {
        return false;
    }

    true
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
