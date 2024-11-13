use std::str::FromStr;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone, Debug, PartialEq)]
enum Number {
    Literal(usize),
    Pair(Box<Number>, Box<Number>),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(n) => write!(f, "{n}"),
            Self::Pair(a, b) => write!(f, "[{a},{b}]"),
        }
    }
}

impl FromStr for Number {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_number(mut chars: &[char]) -> (usize, &[char]) {
            let mut v = 0;
            loop {
                match chars[0] {
                    '0'..='9' => {
                        v = (v * 10) + ((chars[0] as u8 - b'0') as usize);
                        chars = &chars[1..];
                    }
                    _ => return (v, chars),
                }
            }
        }

        fn parse(mut chars: &[char]) -> Result<(Number, &[char]), Error> {
            // Consume [
            if chars[0] != '[' {
                return Err(Error::InvalidInput(format!(
                    "Expected '[', Found {:?}",
                    chars.iter().collect::<String>()
                )));
            }
            chars = &chars[1..];

            let a: Number;
            let b: Number;

            // Parse first number
            match chars[0] {
                '[' => (a, chars) = parse(chars)?,
                _ => {
                    let n: usize;
                    (n, chars) = parse_number(chars);
                    a = Number::Literal(n);
                }
            }

            // Consume ,
            if chars[0] != ',' {
                return Err(Error::InvalidInput(format!(
                    "Expected ',', Found {:?}",
                    chars.iter().collect::<String>()
                )));
            }
            chars = &chars[1..];

            // Parse second number
            match chars[0] {
                '[' => (b, chars) = parse(chars)?,
                _ => {
                    let n: usize;
                    (n, chars) = parse_number(chars);
                    b = Number::Literal(n);
                }
            }

            // Consume ]
            if chars[0] != ']' {
                return Err(Error::InvalidInput(format!(
                    "Expected ']', Found {:?}",
                    chars.iter().collect::<String>()
                )));
            }
            chars = &chars[1..];

            Ok((Number::Pair(Box::new(a), Box::new(b)), chars))
        }

        let chars: Vec<char> = s.chars().collect();
        let (pair, chars) = parse(&chars)?;
        if !chars.is_empty() {
            return Err(Error::InvalidInput(chars.iter().collect()));
        }

        Ok(pair)
    }
}

impl Number {
    fn magnitude(&self) -> usize {
        match self {
            Self::Literal(n) => *n,
            Self::Pair(a, b) => {
                let a = 3 * a.magnitude();
                let b = 2 * b.magnitude();
                a + b
            }
        }
    }

    fn reduce(&mut self) {
        fn inner(
            number: &mut Number,
            depth: usize,
            step1: bool,
        ) -> Result<(Option<Number>, Option<Number>), ()> {
            if let Number::Pair(a, b) = number {
                if step1 && depth == 4 {
                    // Explode
                    let a = a.as_ref().clone();
                    let b = b.as_ref().clone();
                    *number = Number::Literal(0);
                    return Ok((Some(a), Some(b)));
                }

                match a.as_mut() {
                    Number::Literal(v) => {
                        if *v >= 10 && !step1 {
                            // Split
                            *a = Box::new(Number::Pair(
                                Box::new(Number::Literal(*v / 2)),
                                Box::new(Number::Literal((*v + 1) / 2)),
                            ));
                            return Ok((None, None));
                        }
                    }
                    p @ Number::Pair(..) => {
                        if let Ok((left, right)) = inner(p, depth + 1, step1) {
                            if let Some(mut right) = right {
                                right.add(b.as_ref());
                                *b = Box::new(right);
                                return Ok((left, None));
                            } else {
                                return Ok((left, right));
                            }
                        }
                    }
                }

                match b.as_mut() {
                    Number::Literal(v) => {
                        if *v >= 10 && !step1 {
                            // Split
                            *b = Box::new(Number::Pair(
                                Box::new(Number::Literal(*v / 2)),
                                Box::new(Number::Literal((*v + 1) / 2)),
                            ));
                            return Ok((None, None));
                        }
                    }
                    p @ Number::Pair(..) => {
                        if let Ok((left, right)) = inner(p, depth + 1, step1) {
                            if let Some(left) = left {
                                a.as_mut().add(&left);
                                return Ok((None, right));
                            } else {
                                return Ok((left, right));
                            }
                        }
                    }
                }
            }

            Err(())
        }

        loop {
            if inner(self, 0, true).is_ok() {
                continue;
            }
            if inner(self, 0, false).is_ok() {
                continue;
            }
            break;
        }
    }

    fn add(&mut self, rhs: &Number) {
        match (self.clone(), rhs) {
            (Self::Literal(a), Self::Literal(b)) => *self = Self::Literal(a + b),
            (Self::Pair(a, mut b), Self::Literal(_)) => {
                b.add(rhs);
                *self = Self::Pair(a, b);
            }
            (Self::Literal(n), Self::Pair(a, b)) => {
                let mut n = Self::Literal(n);
                n.add(a);
                let b = b.clone();
                *self = Self::Pair(Box::new(n), b);
            }
            (lhs, rhs) => {
                *self = Self::Pair(Box::new(lhs.clone()), Box::new(rhs.clone()));
            }
        }
    }
}

pub struct Day18 {
    numbers: Vec<Number>,
}

impl Day18 {
    pub fn new() -> Self {
        Self {
            numbers: Vec::new(),
        }
    }
}

impl Runner for Day18 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.numbers.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut ans = self.numbers[0].clone();
        for number in &self.numbers[1..] {
            ans.add(number);
            ans.reduce();
        }
        Ok(ans.magnitude().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;
        for a in self.numbers.iter() {
            for b in self.numbers.iter() {
                if a == b {
                    continue;
                }
                let mut a = a.clone();
                a.add(b);
                a.reduce();
                ans = ans.max(a.magnitude());
            }
        }
        Ok(ans.into())
    }
}
