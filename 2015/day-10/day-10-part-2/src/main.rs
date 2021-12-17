#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

#[derive(Clone)]
struct Number {
    digits: Vec<u8>,
}

impl Number {
    fn push(&mut self, digit: u8, num: usize) {
        let mut mag = 1;
        while num / mag != 0 {
            mag *= 10;
        }
        mag /= 10;

        let mut num = num;
        loop {
            self.digits.push((num / mag) as u8);
            num = num % mag;

            if mag == 1 {
                break;
            }
            mag /= 10;
        }

        self.digits.push(digit);
    }

    fn next(&self) -> Self {
        let mut next = Number { digits: Vec::new() };

        let mut cur = self.digits[0];
        let mut cur_len = 1;
        for i in 1..self.digits.len() {
            if self.digits[i] != cur {
                next.push(cur, cur_len);
                cur = self.digits[i];
                cur_len = 1;
            } else {
                cur_len += 1;
            }
        }
        next.push(cur, cur_len);

        next
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for d in &self.digits {
            write!(fmt, "{}", d)?;
        }

        Ok(())
    }
}

fn load_input(filename: &str) -> Result<Vec<Number>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut numbers: Vec<Number> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut number = Number { digits: Vec::new() };
        for c in line.chars() {
            number.digits.push(c as u8 - '0' as u8);
        }

        numbers.push(number);
    }

    Ok(numbers)
}

fn main() -> Result<(), Error> {
    let numbers = load_input(INPUT_FILE)?;

    for number in &numbers {
        let mut cur = number.clone();
        for _ in 0..50 {
            let next = cur.next();
            cur = next;
        }

        println!("len: {}", cur.digits.len());
    }

    Ok(())
}
