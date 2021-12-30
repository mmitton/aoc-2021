#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<Vec<Vec<char>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut inputs = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        inputs.push(line.chars().collect());
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    let mut answer: Vec<char> = Vec::new();

    for i in 0..inputs[0].len() {
        let mut letters: BTreeMap<char, usize> = BTreeMap::new();
        let mut max = 0usize;
        let mut max_letter = ' ';
        for j in 0..inputs.len() {
            let num = *letters.get(&inputs[j][i]).unwrap_or(&0) + 1;
            if num > max {
                max = num;
                max_letter = inputs[j][i];
            }
            letters.insert(inputs[j][i], num);
        }

        answer.push(max_letter);
    }

    println!("Answer: {}", answer.iter().collect::<String>());

    Ok(())
}
