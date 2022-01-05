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
    possible_positions: Vec<usize>,
}

#[derive(Debug)]
struct Input {
    fields: Vec<Field>,
    tickets: Vec<Vec<usize>>,
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
        tickets: Vec::new(),
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
                        possible_positions: Vec::new(),
                    };
                    for range in parts[1].split(" or ") {
                        let nums: Vec<&str> = range.split("-").collect();
                        assert!(nums.len() == 2);
                        field.ranges.push((nums[0].parse()?, nums[1].parse()?));
                    }
                    input.fields.push(field);
                }
                State::MyTicket | State::OtherTickets => {
                    let mut ticket: Vec<usize> = Vec::new();
                    for num in line.split(",") {
                        ticket.push(num.parse()?);
                    }
                    input.tickets.push(ticket);
                }
            }
        }
    }

    Ok(input)
}

fn main() -> Result<(), Error> {
    let mut input = load_input(INPUT_FILE)?;

    macro_rules! is_bad {
        ($ticket:expr) => {{
            let mut good = true;
            for num in $ticket {
                let mut num_good = false;
                for field in &input.fields {
                    for range in &field.ranges {
                        if *num >= range.0 && *num <= range.1 {
                            num_good = true;
                            break;
                        }
                    }
                }
                if !num_good {
                    good = false;
                    break;
                }
            }

            good
        }};
    }

    input.tickets.retain(|t| is_bad!(t));

    for fidx in 0..input.fields.len() {
        'search_loop: for i in 0..input.fields.len() {
            for tidx in 0..input.tickets.len() {
                let num = input.tickets[tidx][i];
                let mut num_good = false;
                for range in &input.fields[fidx].ranges {
                    if num >= range.0 && num <= range.1 {
                        num_good = true;
                        break;
                    }
                }

                if !num_good {
                    continue 'search_loop;
                }
            }

            input.fields[fidx].possible_positions.push(i);
        }
    }

    let mut fixed = Vec::new();
    let mut answer = 1;
    'fix_loop: loop {
        for i in 0..input.fields.len() {
            assert!(input.fields[i].possible_positions.len() > 0);
            if input.fields[i].possible_positions.len() == 1 && !fixed.contains(&i) {
                let keep = input.fields[i].possible_positions[0];
                if input.fields[i].name.starts_with("departure") {
                    answer *= input.tickets[0][keep];
                }
                fixed.push(i);
                for j in 0..input.fields.len() {
                    if i == j {
                        continue;
                    }

                    input.fields[j].possible_positions.retain(|n| *n != keep);
                }
                continue 'fix_loop;
            }
        }

        break;
    }
    for field in &input.fields {
        println!("Field: {:?}", field);
    }

    println!("Answer: {}", answer);
    Ok(())
}
