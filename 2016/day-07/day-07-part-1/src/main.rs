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

    let mut num_tls = 0;
    for input in &inputs {
        let mut in_hypernet = false;
        let mut abba_outside = 0;
        let mut abba_inside = 0;
        for i in 0..input.len() {
            if input[i] == '[' {
                assert!(in_hypernet == false);
                in_hypernet = true;
            } else if input[i] == ']' {
                assert!(in_hypernet == true);
                in_hypernet = false;
            } else if i < input.len() - 3 {
                // Look for ABBA
                if input[i] == input[i + 3]
                    && input[i + 1] == input[i + 2]
                    && input[i] != input[i + 1]
                {
                    if in_hypernet {
                        abba_inside += 1;
                    } else {
                        abba_outside += 1;
                    }
                }
            }
        }
        if abba_inside == 0 && abba_outside != 0 {
            num_tls += 1;
        }
        if cfg!(debug_assertions) {
            println!(
                "{} - {}  {} {}",
                abba_inside == 0 && abba_outside != 0,
                input.iter().collect::<String>(),
                abba_inside,
                abba_outside,
            );
        }
    }

    println!("Number with TLS: {}", num_tls);

    Ok(())
}
