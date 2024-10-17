#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default, Debug)]
enum JSON {
    #[default]
    Empty,
    Number(isize),
    String(String),
    Array(Vec<JSON>),
    Hash(HashMap<String, JSON>),
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
            return Err(Error::InvalidInput("Bad string".into()));
        }

        for i in self.pos + 1..self.chars.len() {
            if self.chars[i] == '"' {
                let ret = self.chars[self.pos + 1..i].iter().collect();
                self.pos = i + 1;
                return Ok(ret);
            }
        }

        Err(Error::InvalidInput("Bad string".into()))
    }
}

impl JSON {
    fn sum(&self, ignore_red: bool) -> isize {
        match self {
            JSON::Number(num) => *num,
            JSON::String(..) => 0,
            JSON::Array(elements) => elements.iter().map(|elem| elem.sum(ignore_red)).sum(),
            JSON::Hash(elements) => {
                let mut sum = 0;
                for elem in elements.iter() {
                    if ignore_red {
                        if let JSON::String(s) = elem.1 {
                            if s == "red" {
                                return 0;
                            }
                        }
                    }
                    sum += elem.1.sum(ignore_red);
                }
                sum
            }
            JSON::Empty => 0,
        }
    }

    fn parse(&mut self, buf: &mut CharArray) -> Result<(), Error> {
        match buf.peek() {
            '"' => {
                // Parse String
                *self = JSON::String(buf.consume_string()?);
                Ok(())
            }
            '[' => {
                // Parse Array
                buf.consume();
                let mut elements = Vec::new();
                if buf.peek() != ']' {
                    loop {
                        let mut element = JSON::Empty;
                        element.parse(buf)?;
                        elements.push(element);
                        match buf.peek() {
                            ',' => {
                                buf.consume();
                            }
                            ']' => {
                                break;
                            }
                            _ => {
                                return Err(Error::InvalidInput("Bad Array".into()));
                            }
                        }
                    }
                }
                // Consume ]
                buf.consume();
                *self = JSON::Array(elements);
                Ok(())
            }
            '{' => {
                // Parse Hash
                buf.consume();
                let mut elements = HashMap::default();
                if buf.peek() != '}' {
                    loop {
                        let name = buf.consume_string()?;
                        if buf.peek() != ':' {
                            return Err(Error::InvalidInput("Bad Hash".into()));
                        }
                        buf.consume();
                        let mut val = JSON::Empty;
                        val.parse(buf)?;

                        elements.insert(name, val);
                        match buf.peek() {
                            ',' => {
                                buf.consume();
                            }
                            '}' => break,
                            _ => return Err(Error::InvalidInput("Bad Hash".into())),
                        }
                    }
                }
                // Consume {
                buf.consume();
                *self = JSON::Hash(elements);
                Ok(())
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

                let num: isize = num.iter().collect::<String>().parse()?;
                *self = JSON::Number(num);
                Ok(())
            }
        }
    }
}

#[derive(Default)]
pub struct Day12 {
    json: JSON,
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;

        for line in lines.iter() {
            let mut buf = CharArray {
                chars: line.chars().collect(),
                pos: 0,
            };
            self.json.parse(&mut buf)?;
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.json.sum(false).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.json.sum(true).into())
    }
}
