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

    for input in &inputs {
        if cfg!(debug_assertions) {
            println!("input: {:?}", input);
        }
        let mut floor = 0isize;
        for (i, c) in input.iter().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => {
                    floor -= 1;
                    if floor == -1 {
                        println!("Entered basement at {}", i + 1);
                        break;
                    }
                }
                _ => panic!(),
            }
        }
    }

    Ok(())
}
