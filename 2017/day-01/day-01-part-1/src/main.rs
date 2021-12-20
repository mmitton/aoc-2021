#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Captcha {
    data: Vec<usize>,
}

impl Captcha {
    fn sum(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.data.len() {
            if self.data[i] == self.data[(i + 1) % self.data.len()] {
                sum += self.data[i];
            }
        }

        sum
    }
}

fn load_input(filename: &str) -> Result<Vec<Captcha>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut captchas = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut captcha = Captcha { data: Vec::new() };
        for i in 0..line.len() {
            captcha
                .data
                .push(line[i..i + 1].parse().map_err(|e| Error::NAN(e))?);
        }

        captchas.push(captcha);
    }

    Ok(captchas)
}

fn main() -> Result<(), Error> {
    let captchas = load_input(INPUT_FILE)?;

    for captcha in captchas {
        println!("Sum: {}  Captcha: {:?}", captcha.sum(), captcha);
    }

    Ok(())
}
