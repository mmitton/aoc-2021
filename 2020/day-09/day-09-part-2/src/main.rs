#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    NoSolution,
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

fn load_input(filename: &str) -> Result<(usize, Vec<usize>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut numbers = Vec::new();
    let mut preamble = None;

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        let number = line.parse()?;
        if preamble.is_none() {
            preamble = Some(number);
        } else {
            numbers.push(number);
        }
    }

    Ok((preamble.unwrap(), numbers))
}

fn main() -> Result<(), Error> {
    let (preamble, mut numbers) = load_input(INPUT_FILE)?;

    'search_loop: for i in preamble..numbers.len() {
        if cfg!(debug_assertions) {
            println!("Considering {}", numbers[i]);
        }
        for j in i - preamble..i {
            for k in j + 1..i {
                if numbers[j] + numbers[k] == numbers[i] {
                    continue 'search_loop;
                }
            }
        }

        println!(
            "{} is invalid, searching for sequence of numbers that add up to it.",
            numbers[i]
        );
        'inner_search: for j in 0..i {
            let mut sum = numbers[j];
            for k in j + 1..i {
                sum += numbers[k];
                if sum > numbers[i] {
                    continue 'inner_search;
                }
                if sum == numbers[i] {
                    let sum_numbers = &mut numbers[j..k + 1];
                    sum_numbers.sort();
                    println!("{:?} = {}", sum_numbers, sum);
                    println!(
                        "Answer: {}",
                        sum_numbers[0] + sum_numbers[sum_numbers.len() - 1]
                    );
                    return Ok(());
                }
            }
        }
        break;
    }

    Err(Error::NoSolution)
}
