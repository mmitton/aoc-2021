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

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct Input {
    fields: Vec<Field>,
    my_ticket: Vec<usize>,
    other_tickets: Vec<Vec<usize>>,
}

fn load_input(filename: &str) -> Result<Input, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let reader = BufReader::new(f);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    enum State {
        Fields,
        MyTicket,
        OtherTickets,
    }

    let mut state = State::Fields;
    let mut input = Input {
        fields: Vec::new(),
        my_ticket: Vec::new(),
        other_tickets: Vec::new(),
    };

    for line in lines {
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        if line == "your ticket:" {
            state = State::MyTicket;
        } else if line == "nearby tickets:" {
            state = State::OtherTickets;
        } else {
            match state {
                State::Fields => {
                    let parts: Vec<&str> = line.split(": ").collect();
                    let mut field = Field {
                        name: parts[0].to_string(),
                        ranges: Vec::new(),
                    };
                    for range in parts[1].split(" or ") {
                        let nums: Vec<&str> = range.split("-").collect();
                        assert!(nums.len() == 2);
                        field.ranges.push((nums[0].parse()?, nums[1].parse()?));
                    }
                    input.fields.push(field);
                }
                State::MyTicket => {
                    for num in line.split(",") {
                        input.my_ticket.push(num.parse()?);
                    }
                }
                State::OtherTickets => {
                    let mut other_ticket: Vec<usize> = Vec::new();
                    for num in line.split(",") {
                        other_ticket.push(num.parse()?);
                    }
                    input.other_tickets.push(other_ticket);
                }
            }
        }
    }

    Ok(input)
}

fn main() -> Result<(), Error> {
    let input = load_input(INPUT_FILE)?;

    let mut bad_numbers: Vec<usize> = Vec::new();
    let mut answer = 0;

    macro_rules! find_bad_numbers {
        ($ticket:expr) => {{
            for num in $ticket {
                let mut good = false;
                for field in &input.fields {
                    for range in &field.ranges {
                        if *num >= range.0 && *num <= range.1 {
                            good = true;
                            break;
                        }
                    }
                }
                if !good {
                    bad_numbers.push(*num);
                    answer += num;
                }
            }
        }};
    }

    find_bad_numbers!(&input.my_ticket);
    for ticket in &input.other_tickets {
        find_bad_numbers!(ticket);
    }

    println!("bad_numbers: {:?}", bad_numbers);
    println!("answer: {}", answer);
    Ok(())
}
