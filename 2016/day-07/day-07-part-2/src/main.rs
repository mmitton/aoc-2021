#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

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

    let mut num_ssl = 0;
    for input in &inputs {
        let mut outside: Vec<[char; 3]> = Vec::new();
        let mut inside: Vec<[char; 3]> = Vec::new();

        let mut in_hypernet = false;
        for i in 0..input.len() {
            if input[i] == '[' {
                assert!(in_hypernet == false);
                in_hypernet = true;
            } else if input[i] == ']' {
                assert!(in_hypernet == true);
                in_hypernet = false;
            } else if i < input.len() - 2 {
                // Look for ABBA
                if input[i] == input[i + 2] && input[i] != input[i + 1] {
                    let triplet = [input[i], input[i + 1], input[i + 2]];
                    if in_hypernet {
                        inside.push(triplet);
                    } else {
                        outside.push(triplet);
                    }
                }
            }
        }

        for triplet in &outside {
            let invert = [triplet[1], triplet[0], triplet[1]];
            if inside.contains(&invert) {
                num_ssl += 1;
                break;
            }
        }
    }

    println!("Number with TLS: {}", num_ssl);

    Ok(())
}
