#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

fn load_input(filename: &str) -> Result<Vec<[usize; 3]>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut inputs = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let mut line = line.trim().to_string();
        if line == "" {
            continue;
        }

        for _ in 0..5 {
            line = line.replace("  ", " ");
        }

        let parts: Vec<&str> = line.split(" ").collect();
        inputs.push([parts[0].parse()?, parts[1].parse()?, parts[2].parse()?]);
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    let mut good = 0usize;
    for r in (0..inputs.len()).step_by(3) {
        for c in 0..3 {
            if inputs[r][c] + inputs[r + 1][c] > inputs[r + 2][c]
                && inputs[r][c] + inputs[r + 2][c] > inputs[r + 1][c]
                && inputs[r + 1][c] + inputs[r + 2][c] > inputs[r][c]
            {
                println!("{:?}", &[inputs[r][c], inputs[r + 1][c], inputs[r + 2][c]]);
                good += 1;
            }
        }
    }

    println!("Good: {}", good);

    Ok(())
}
