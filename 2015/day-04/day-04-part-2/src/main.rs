use md5;

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<Vec<String>, Error> {
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

        inputs.push(line.to_string());
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    for input in &inputs {
        let initial_input = input.as_bytes();
        let mut input = Vec::new();

        for i in 0..u32::MAX {
            input.clear();
            input.extend_from_slice(&initial_input);
            input.extend_from_slice(i.to_string().as_bytes());
            let result = md5::compute(&input);

            if result[0] == 0 && result[1] == 0 && result[2] == 0 {
                println!("i: {}  result: {:?}", i, result);
                break;
            }
        }
    }

    Ok(())
}
