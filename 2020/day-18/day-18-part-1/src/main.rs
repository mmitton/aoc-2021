#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::VecDeque;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    IncompleteExpression,
    InvalidOp(String),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

fn consume_expression(tokens: &mut VecDeque<&str>) -> Result<usize, Error> {
    let mut result = consume_number(tokens)?;
    while tokens.len() != 0 {
        let operation = tokens.pop_front().unwrap();
        match operation {
            "+" => {
                let next = consume_number(tokens)?;
                result += next;
            }
            "*" => {
                let next = consume_number(tokens)?;
                result *= next;
            }
            ")" => break,
            _ => return Err(Error::InvalidOp(operation.to_string())),
        }
    }

    Ok(result)
}

fn consume_number(tokens: &mut VecDeque<&str>) -> Result<usize, Error> {
    if tokens.len() == 0 {
        return Err(Error::IncompleteExpression);
    }
    let next = tokens.pop_front().unwrap();

    if next == "(" {
        consume_expression(tokens)
    } else {
        let num = next.parse()?;
        Ok(num)
    }
}

fn load_input(filename: &str) -> Result<Vec<(String, usize)>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut expressions = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        let expression = line.to_string();
        let tokens = line.replace("(", "( ");
        let tokens = tokens.replace(")", " )");
        let mut tokens: VecDeque<&str> = tokens.split(" ").collect();
        let result = consume_expression(&mut tokens)?;

        expressions.push((expression, result));
    }

    Ok(expressions)
}

fn main() -> Result<(), Error> {
    let expressions = load_input(INPUT_FILE)?;

    let mut sum = 0;
    for expression in &expressions {
        println!("{} = {}", expression.0, expression.1);
        sum += expression.1;
    }

    println!("Answer: {}", sum);

    Ok(())
}
