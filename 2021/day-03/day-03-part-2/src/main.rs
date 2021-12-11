#![feature(drain_filter)]

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    NoSolution,
}

#[derive(Debug, Clone)]
struct Number {
    val: Vec<char>,
    maybe_oxygen: bool,
    maybe_co2: bool,
}

enum Filter {
    Oxygen,
    CO2,
}

impl Number {
    fn as_usize(&self) -> usize {
        let mut v = 0usize;
        for c in &self.val {
            v <<= 1;
            if *c == '1' {
                v |= 1;
            }
        }
        v
    }
}

#[derive(Debug, Clone)]
struct Report {
    numbers: Vec<Number>,
}

impl Report {
    fn count_bit(&self, bit: usize, c: char) -> usize {
        let mut count = 0usize;
        for number in &self.numbers {
            if number.val[bit] == c {
                count += 1;
            }
        }
        count
    }

    fn filter_oxygen(&mut self, bit: usize, c: char) -> Option<usize> {
        for number in &mut self.numbers {
            if number.maybe_oxygen && number.val[bit] != c {
                number.maybe_oxygen = false;
            }
        }

        // Remove all
        self.numbers.drain_filter(|n| !n.maybe_oxygen);

        if self.numbers.len() == 1 {
            return Some(self.numbers[0].as_usize());
        }

        None
    }

    fn filter_co2(&mut self, bit: usize, c: char) -> Option<usize> {
        for number in &mut self.numbers {
            if number.maybe_co2 && number.val[bit] != c {
                number.maybe_co2 = false;
            }
        }

        // Remove all
        self.numbers.drain_filter(|n| !n.maybe_co2);

        if self.numbers.len() == 1 {
            return Some(self.numbers[0].as_usize());
        }

        None
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
            if chars.len() != report.numbers[0].val.len() {
                return Err(Error::InvalidInput(line.to_string()));
            }
        }

        report.numbers.push(Number {
            val: chars,
            maybe_co2: true,
            maybe_oxygen: true,
        });
    }

    Ok(report)
}

fn main() -> Result<(), Error> {
    let mut report = load_report(INPUT_FILE)?;
    let mut report_oxygen = report.clone();
    let mut report_co2 = report.clone();

    let mut gamma = 0usize;
    let mut epsilon = 0usize;
    let mut oxygen: Option<usize> = None;
    let mut co2: Option<usize> = None;

    for i in 0..report.numbers[0].val.len() {
        gamma <<= 1;
        epsilon <<= 1;

        let zero_count = report.count_bit(i, '0');
        let one_count = report.count_bit(i, '1');

        if one_count > zero_count {
            gamma |= 1;
        } else {
            epsilon |= 1;
        };

        if oxygen.is_none() {
            let zero_count = report_oxygen.count_bit(i, '0');
            let one_count = report_oxygen.count_bit(i, '1');

            let mc = if one_count >= zero_count { '1' } else { '0' };
            oxygen = report_oxygen.filter_oxygen(i, mc);
        }

        if co2.is_none() {
            let zero_count = report_co2.count_bit(i, '0');
            let one_count = report_co2.count_bit(i, '1');

            let mc = if one_count < zero_count { '1' } else { '0' };
            co2 = report_co2.filter_co2(i, mc);
        }
    }

    println!(
        "gamma:{}  epsilon:{}  answer:{}  oxygen:{:?}  co2:{:?}",
        gamma,
        epsilon,
        gamma * epsilon,
        oxygen,
        co2
    );

    match (oxygen, co2) {
        (Some(oxygen), Some(co2)) => println!("answer:{}", oxygen * co2),
        _ => return Err(Error::NoSolution),
    }

    Ok(())
}
