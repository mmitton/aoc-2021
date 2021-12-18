#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
}

#[derive(Debug, Clone)]
enum Token {
    OpenBracket,
    CloseBracket,
    Number(usize),
}

#[derive(Debug, Clone)]
struct Number {
    tokens: Vec<Token>,
}

impl std::fmt::Display for Number {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.tokens.len() {
            match self.tokens[i] {
                Token::OpenBracket => {
                    write!(fmt, "[")?;
                }
                Token::CloseBracket => {
                    write!(fmt, "]")?;
                }
                Token::Number(n) => {
                    write!(fmt, "{}", n)?;
                }
            }
            write!(fmt, " ")?;
        }

        Ok(())
    }
}

impl Number {
    fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut num = Number { tokens: Vec::new() };
        let c: Vec<char> = s.chars().collect();

        let mut i = 0usize;
        loop {
            if i == c.len() {
                break;
            }
            assert!(i < c.len());

            match c[i] {
                '[' => {
                    num.tokens.push(Token::OpenBracket);
                }
                ']' => {
                    num.tokens.push(Token::CloseBracket);
                }
                ',' => {}
                _ => {
                    // Consume a number!
                    let start = i;
                    let mut end = 0usize;
                    for j in i..c.len() {
                        if !c[j].is_ascii_digit() {
                            end = j - 1;
                            break;
                        }
                    }

                    if end < start {
                        return Err(Error::InvalidInput(s[i..].to_string()));
                    }

                    num.tokens
                        .push(Token::Number(s[start..end + 1].parse().unwrap()));
                    i = end;
                }
            }
            i += 1;
        }

        Ok(num)
    }

    fn add(&mut self, rhs: &Number) {
        if self.tokens.len() == 0 {
            // Just clone tokens
            self.tokens = rhs.tokens.clone();
            return;
        }

        self.tokens.insert(0, Token::OpenBracket);
        for t in &rhs.tokens {
            self.tokens.push(t.clone());
        }
        self.tokens.push(Token::CloseBracket);

        self.reduce();
    }

    fn reduce(&mut self) {
        while self.reduce_inner() {}
    }

    fn reduce_inner(&mut self) -> bool {
        fn get_number(tokens: &Vec<Token>, pos: usize, delta: isize) -> Option<(usize, usize)> {
            let mut idx = pos as isize;
            loop {
                idx += delta;
                if idx < 0 {
                    return None;
                }
                if idx as usize >= tokens.len() {
                    return None;
                }

                if let Token::Number(n) = tokens[idx as usize] {
                    return Some((idx as usize, n));
                }
            }
        }

        let mut depth = 0usize;
        let mut i = 0usize;
        loop {
            if i == self.tokens.len() {
                break;
            }
            assert!(i < self.tokens.len());

            match self.tokens[i] {
                Token::OpenBracket => depth += 1,
                Token::CloseBracket => depth -= 1,
                Token::Number(n1) => {
                    if depth > 4 {
                        if let Token::Number(n2) = self.tokens[i + 1] {
                            // EXPLODE

                            let last_num = get_number(&self.tokens, i, -1);
                            let next_num = get_number(&self.tokens, i + 1, 1);

                            if let Some((idx, num)) = last_num {
                                self.tokens[idx] = Token::Number(num + n1);
                            }
                            if let Some((idx, num)) = next_num {
                                self.tokens[idx] = Token::Number(num + n2);
                            }

                            self.tokens[i - 1] = Token::Number(0);
                            self.tokens.remove(i + 2);
                            self.tokens.remove(i + 1);
                            self.tokens.remove(i);

                            return true;
                        }
                    }
                }
            }

            i += 1;
        }

        i = 0usize;
        loop {
            if i == self.tokens.len() {
                break;
            }
            assert!(i < self.tokens.len());

            match self.tokens[i] {
                Token::Number(n1) => {
                    if n1 >= 10 {
                        let lhs = n1 / 2;
                        let rhs = (n1 + 1) / 2;
                        self.tokens[i] = Token::OpenBracket;
                        self.tokens.insert(i + 1, Token::Number(lhs));
                        self.tokens.insert(i + 2, Token::Number(rhs));
                        self.tokens.insert(i + 3, Token::CloseBracket);

                        return true;
                    }
                }
                _ => {}
            }
            i += 1
        }

        false
    }

    fn magnitude(&self) -> usize {
        fn mag_reduce(tokens: &mut Vec<Token>) -> bool {
            for i in 0..tokens.len() - 1 {
                match (&tokens[i], &tokens[i + 1]) {
                    (Token::Number(n1), Token::Number(n2)) => {
                        tokens[i] = Token::Number((3 * n1) + (2 * n2));
                        tokens.remove(i + 2);
                        tokens.remove(i + 1);
                        tokens.remove(i - 1);
                        return true;
                    }
                    _ => {}
                }
            }

            false
        }

        let mut tokens = self.tokens.clone();
        while mag_reduce(&mut tokens) {}

        assert!(tokens.len() == 1);

        if let Token::Number(n) = tokens[0] {
            return n;
        }

        panic!("wtf?!");
    }
}

fn load_input(filename: &str) -> Result<Vec<Number>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut ret = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        ret.push(Number::from_str(line)?);
    }

    Ok(ret)
}

fn main() -> Result<(), Error> {
    let numbers = load_input(INPUT_FILE)?;

    fn magnitude(t1: Number, t2: Number) -> usize {
        let mut number = Number::new();
        number.add(&t1);
        number.add(&t2);
        number.magnitude()
    }

    let mut best = 0usize;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if j == i {
                continue;
            }

            let mag_1 = magnitude(numbers[i].clone(), numbers[j].clone());
            let mag_2 = magnitude(numbers[j].clone(), numbers[i].clone());

            if mag_1 > best {
                best = mag_1;
            }
            if mag_2 > best {
                best = mag_2;
            }
        }
    }

    println!("Best: {}", best);

    Ok(())
}
