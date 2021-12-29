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
        let line = line.trim();
        if line == "" {
            continue;
        }

        inputs.push(line.chars().collect());
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    let keypad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut x = 1;
    let mut y = 1;
    let mut answer = "".to_string();

    for input in &inputs {
        for c in input {
            match c {
                'U' => {
                    if y > 0 {
                        y -= 1
                    }
                }
                'D' => {
                    if y < 2 {
                        y += 1
                    }
                }
                'L' => {
                    if x > 0 {
                        x -= 1
                    }
                }
                'R' => {
                    if x < 2 {
                        x += 1
                    }
                }
                _ => unreachable!(),
            }
        }

        answer.push_str(&format!("{}", keypad[y][x]));
    }
    println!("Answer: {}", answer);

    Ok(())
}
