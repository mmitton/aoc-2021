use std::collections::BTreeMap;

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

#[derive(Debug)]
struct Word {
    word: String,
    chars: BTreeMap<char, usize>,
}

impl Word {
    fn from_str(s: &str) -> Word {
        let mut chars = BTreeMap::new();
        for c in s.chars() {
            let num = *chars.get(&c).unwrap_or(&0) + 1;
            chars.insert(c, num);
        }

        Self {
            word: s.to_string(),
            chars: chars,
        }
    }

    fn is_anagram(&self, rhs: &Self) -> bool {
        if self.word.len() != rhs.word.len() {
            return false;
        }

        for c in self.chars.keys() {
            let n1 = *self.chars.get(&c).unwrap();
            let n2 = *rhs.chars.get(&c).unwrap_or(&0);

            if n1 != n2 {
                return false;
            }
        }

        return true;
    }
}

#[derive(Debug)]
struct Passphrase {
    words: Vec<Word>,
    is_valid: bool,
}

impl Passphrase {
    fn from_str(s: &str) -> Passphrase {
        let mut words: Vec<Word> = Vec::new();
        let mut is_valid = true;

        for word in s.split(" ") {
            let word = Word::from_str(word);
            for w in &words {
                if w.is_anagram(&word) {
                    is_valid = false;
                    break;
                }
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
        println!("{}  - {:?}", passphrase.is_valid, passphrase.words);
        if passphrase.is_valid {
            num_good += 1;
        }
    }

    println!("Num good: {}", num_good);

    Ok(())
}
