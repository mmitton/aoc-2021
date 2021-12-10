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

    let mut ret = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        ret.push(line.chars().collect::<Vec<char>>());
    }

    Ok(ret)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    let mut answer = 0usize;
    'input_loop: for input in inputs {
        let mut stack = Vec::new();
        for c in input {
            let lookfor = match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                    None
                }
                ')' => Some(('(', 3)),
                ']' => Some(('[', 57)),
                '}' => Some(('{', 1197)),
                '>' => Some(('<', 25137)),
                _ => panic!("unexpected input '{}'", c),
            };

            if let Some((lookfor, points)) = lookfor {
                let last = stack.pop().expect("Stack empty");
                if last != lookfor {
                    answer += points;
                    continue 'input_loop;
                }
            }
        }
    }

    println!("Answer: {}", answer);
    Ok(())
}
