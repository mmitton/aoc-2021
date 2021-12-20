#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

#[derive(Debug)]
struct Passphrase {
    words: Vec<String>,
    is_valid: bool,
}

impl Passphrase {
    fn from_str(s: &str) -> Passphrase {
        let mut words = Vec::new();
        let mut is_valid = true;

        for word in s.split(" ") {
            let word = word.to_string();
            if words.contains(&word) {
                is_valid = false;
            }
            words.push(word);
        }

        Self {
            words: words,
            is_valid: is_valid,
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<Passphrase>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut passphrases = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        passphrases.push(Passphrase::from_str(line));
    }

    Ok(passphrases)
}

fn main() -> Result<(), Error> {
    let passphrases = load_input(INPUT_FILE)?;

    let mut num_good = 0;
    for passphrase in &passphrases {
        println!("{:?}", passphrase);
        if passphrase.is_valid {
            num_good += 1;
        }
    }

    println!("Num good: {}", num_good);

    Ok(())
}
