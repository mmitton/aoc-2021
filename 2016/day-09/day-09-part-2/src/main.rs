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

fn marker(input: &[char], i: usize) -> Option<(usize, usize, usize)> {
    if input[i] == '(' {
        let mut x = 0;
        for j in i + 1..input.len() {
            if input[j] == 'x' {
                x = j;
                break;
            } else if !input[j].is_digit(10) {
                return None;
            }
        }
        if x == 0 {
            return None;
        }

        let mut close = 0;
        for j in x + 1..input.len() {
            if input[j] == ')' {
                close = j;
                break;
            } else if !input[j].is_digit(10) {
                return None;
            }
        }
        if close == 0 {
            return None;
        }

        let num: usize = input[i + 1..x].iter().collect::<String>().parse().unwrap();
        let repeat: usize = input[x + 1..close]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

        Some((num, repeat, close + 1))
    } else {
        None
    }
}

fn decompress(input: &[char]) -> (usize, Vec<char>) {
    let mut decompressed_len = 0;
    let mut decompressed = Vec::new();
    let mut i = 0;
    while i < input.len() {
        if let Some((num, repeat, end)) = marker(input, i) {
            let (sublen, sub) = decompress(&input[end..end + num]);
            decompressed_len += sublen * repeat;
            for _ in 0..repeat {
                decompressed.extend_from_slice(&sub);
            }
            i = end + num;
        } else {
            decompressed_len += 1;
            if cfg!(debug_assertions) {
                decompressed.push(input[i]);
            }
            i += 1;
        }
    }

    (decompressed_len, decompressed)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    for input in &inputs {
        let (decompressed_len, decompressed) = decompress(&input[..]);
        if cfg!(debug_assertions) {
            println!(
                "input: '{}'  decompressed: {}",
                input.iter().collect::<String>(),
                decompressed.iter().collect::<String>(),
            );
            assert!(decompressed.len() == decompressed_len);
        }
        println!("Decompressed Length: {}", decompressed_len);
    }

    Ok(())
}
