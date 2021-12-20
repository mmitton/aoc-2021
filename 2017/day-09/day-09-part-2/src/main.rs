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
    let mut streams: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        let mut chars: Vec<char> = Vec::new();
        let line_chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < line_chars.len() {
            if line_chars[i] == '!' {
                i += 1;
            } else {
                chars.push(line_chars[i]);
            }

            i += 1;
        }

        streams.push(chars);
    }

    Ok(streams)
}

fn main() -> Result<(), Error> {
    let streams = load_input(INPUT_FILE)?;

    for stream in &streams {
        if cfg!(debug_assertions) {
            println!("{:?}", stream);
        }
        let mut depth = 0;
        let mut score = 0;
        let mut in_garbage = false;
        let mut garbage = 0;
        for c in stream {
            if in_garbage {
                if *c == '>' {
                    in_garbage = false;
                } else {
                    garbage += 1;
                }
            } else {
                if *c == '{' {
                    depth += 1;
                    score += depth;
                } else if *c == '}' {
                    depth -= 1;
                } else if *c == '<' {
                    in_garbage = true;
                }
            }
        }

        println!("Score: {}  Garbage: {}", score, garbage);
    }

    Ok(())
}
