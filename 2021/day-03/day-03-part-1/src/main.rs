#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
}

#[derive(Debug)]
struct Report {
    numbers: Vec<Vec<char>>,
}

impl Report {
    fn count_bit(&self, bit: usize, c: char) -> usize {
        let mut count = 0usize;
        for number in &self.numbers {
            if number[bit] == c {
                count += 1;
            }
        }
        count
    }
}

fn load_report(filename: &str) -> Result<Report, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let mut report = Report {
        numbers: Vec::new(),
    };
    for line in BufReader::new(f).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let chars = line.chars().collect::<Vec<char>>();
        if report.numbers.len() != 0 {
            if chars.len() != report.numbers[0].len() {
                return Err(Error::InvalidInput(line.to_string()));
            }
        }

        report.numbers.push(chars);
    }

    Ok(report)
}

fn main() {
    let report = load_report(INPUT_FILE).expect("Could not load input file");

    let mut gamma = 0usize;
    let mut epsilon = 0usize;
    for i in 0..report.numbers[0].len() {
        gamma <<= 1;
        epsilon <<= 1;

        let zero_count = report.count_bit(i, '0');
        let one_count = report.count_bit(i, '1');

        if one_count > zero_count {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    println!(
        "gamma:{}  epsilon:{}  answer:{}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
