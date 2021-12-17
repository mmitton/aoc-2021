use std::collections::BTreeMap;

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample-2.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidArray,
    InvalidHash,
    InvalidString,
}

#[derive(Debug)]
enum JSON {
    Number(isize),
    String(String),
    Array(Vec<JSON>),
    Hash(BTreeMap<String, JSON>),
}

struct CharArray {
    chars: Vec<char>,
    pos: usize,
}

impl CharArray {
    fn peek(&self) -> char {
        if self.pos == self.chars.len() {
            return 0 as char;
        }
        self.chars[self.pos]
    }

    fn consume(&mut self) -> char {
        let ret = self.chars[self.pos];
        self.pos += 1;
        ret
    }

    fn consume_string(&mut self) -> Result<String, Error> {
        if self.chars[self.pos] != '"' {
            return Err(Error::InvalidString);
        }

        for i in self.pos + 1..self.chars.len() {
            if self.chars[i] == '"' {
                let ret = self.chars[self.pos + 1..i].iter().collect();
                self.pos = i + 1;
                return Ok(ret);
            }
        }

        return Err(Error::InvalidString);
    }
}

impl JSON {
    fn sum(&self) -> isize {
        let mut res = 0isize;
        match self {
            JSON::Number(num) => res = *num,
            JSON::String(..) => res = 0,
            JSON::Array(elements) => {
                for elem in elements {
                    res += elem.sum();
                }
            }
            JSON::Hash(elements) => {
                for elem in elements {
                    if let JSON::String(s) = elem.1 {
                        if s == "red" {
                            return 0;
                        }
                    }
                    res += elem.1.sum();
                }
            }
        }

        res
    }

    fn parse(buf: &mut CharArray) -> Result<JSON, Error> {
        match buf.peek() {
            '"' => {
                // Parse String
                Ok(JSON::String(buf.consume_string()?))
            }
            '[' => {
                // Parse Array
                buf.consume();
                let mut elements = Vec::new();
                if buf.peek() != ']' {
                    loop {
                        elements.push(JSON::parse(buf)?);
                        match buf.peek() {
                            ',' => {
                                buf.consume();
                            }
                            ']' => {
                                break;
                            }
                            _ => {
                                println!(".. '{}'", buf.peek());
                                return Err(Error::InvalidArray);
                            }
                        }
                    }
                }
                // Consume ]
                buf.consume();
                Ok(JSON::Array(elements))
            }
            '{' => {
                // Parse Hash
                buf.consume();
                let mut elements = BTreeMap::new();
                if buf.peek() != '}' {
                    loop {
                        let name = buf.consume_string()?;
                        if buf.peek() != ':' {
                            return Err(Error::InvalidHash);
                        }
                        buf.consume();
                        let val = JSON::parse(buf)?;

                        elements.insert(name, val);
                        match buf.peek() {
                            ',' => {
                                buf.consume();
                            }
                            '}' => break,
                            _ => return Err(Error::InvalidHash),
                        }
                    }
                }
                // Consume {
                buf.consume();
                Ok(JSON::Hash(elements))
            }
            _ => {
                // Parse Number
                let mut num = Vec::new();
                loop {
                    let next = buf.peek();
                    if next == '-' || next.is_ascii_digit() {
                        num.push(buf.consume());
                    } else {
                        break;
                    }
                }

                let num: isize = num
                    .iter()
                    .collect::<String>()
                    .parse()
                    .map_err(|e| Error::NAN(e))?;
                Ok(JSON::Number(num))
            }
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<JSON>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut inputs: Vec<JSON> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut buf = CharArray {
            chars: line.chars().collect(),
            pos: 0,
        };

        let input = JSON::parse(&mut buf);
        inputs.push(input?);
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    for input in &inputs {
        if cfg!(debug_assertions) {
            println!("input: {:?}  sum: {}", input, input.sum());
        } else {
            println!("sum: {}", input.sum());
        }
    }

    Ok(())
}
