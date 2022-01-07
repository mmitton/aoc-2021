#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample-2.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NoSolution,
}

fn load_input(filename: &str) -> Result<Vec<Vec<char>>, Error> {
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

        ids.push(line.chars().collect());
    }

    Ok(ids)
}

fn main() -> Result<(), Error> {
    let ids = load_input(INPUT_FILE)?;

    let mut common = String::new();

    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            if ids[i].len() != ids[j].len() {
                continue;
            }

            common.clear();
            let mut num_off = 0;
            for k in 0..ids[i].len() {
                if ids[i][k] == ids[j][k] {
                    common.push(ids[i][k]);
                    continue;
                }
                num_off += 1;
            }

            if num_off == 1 {
                println!(
                    "Found boxes: {} {}",
                    ids[i].iter().collect::<String>(),
                    ids[j].iter().collect::<String>()
                );
                println!("Answer: {}", common);
                return Ok(());
            }
        }
    }

    Err(Error::NoSolution)
}
