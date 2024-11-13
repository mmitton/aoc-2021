use std::str::FromStr;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day18 {
    expressions: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
enum Token {
    Add,
    Mul,
    Open,
    Close,
    Num(usize),
}

impl FromStr for Token {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            "(" => Ok(Self::Open),
            ")" => Ok(Self::Close),
            _ => Ok(Self::Num(s.parse()?)),
        }
    }
}

#[derive(Debug)]
enum Expression {
    Num(usize),
    Add,
    Mul,
    Group(Vec<Expression>),
}

impl Expression {
    fn eval(&self) -> usize {
        match self {
            Self::Num(n) => *n,
            Self::Group(e) => {
                assert_eq!(e.len(), 3);
                let a = e[0].eval();
                let b = e[2].eval();
                match e[1] {
                    Self::Add => a + b,
                    Self::Mul => a * b,
                    Self::Num(..) | Self::Group(..) => unreachable!(),
                }
            }
            Self::Add | Self::Mul => unreachable!(),
        }
    }
}

impl From<&[Token]> for Expression {
    fn from(tokens: &[Token]) -> Self {
        use std::iter::Peekable;
        use std::slice::Iter;
        fn consume(tokens: &mut Peekable<Iter<Token>>) -> Expression {
            // Consume an expression, returning the remainder
            let mut expression = Vec::new();
            while let Some(token) = tokens.next_if(|&token| !matches!(token, Token::Close)) {
                expression.push(match token {
                    Token::Add => Expression::Add,
                    Token::Mul => Expression::Mul,
                    Token::Num(n) => Expression::Num(*n),
                    Token::Open => {
                        let inner = consume(tokens);
                        assert_eq!(tokens.next(), Some(&Token::Close));
                        inner
                    }
                    Token::Close => unreachable!(),
                });
            }
            Expression::Group(expression)
        }

        let mut iter = tokens.iter().peekable();
        let expr = consume(&mut iter);
        assert!(iter.next().is_none());
        expr
    }
}

impl Day18 {
    pub fn new() -> Self {
        Self {
            expressions: Vec::new(),
        }
    }
}

impl Runner for Day18 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let line = line.replace('(', " ( ");
            let line = line.replace(')', " ) ");
            let tokens: Vec<Token> = line
                .split_whitespace()
                .map(|t| t.parse::<Token>().unwrap())
                .collect();
            let expr: Expression = tokens.as_slice().into();
            self.expressions.push(expr);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        fn group(expr: &mut Expression) {
            match expr {
                Expression::Group(e) => {
                    for e in e.iter_mut().step_by(2) {
                        group(e);
                    }
                    while e.len() != 3 {
                        let a = e.remove(0);
                        let op = e.remove(0);
                        let b = e.remove(0);

                        assert!(matches!(op, Expression::Add | Expression::Mul));

                        e.insert(0, Expression::Group(vec![a, op, b]));
                    }
                }
                Expression::Num(_) => {}
                Expression::Add | Expression::Mul => unreachable!(),
            }
        }

        Ok(self
            .expressions
            .iter_mut()
            .fold(0, |acc, expr| {
                group(expr);
                acc + expr.eval()
            })
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        fn group(expr: &mut Expression) {
            match expr {
                Expression::Group(e) => {
                    for e in e.iter_mut().step_by(2) {
                        group(e);
                    }
                    for idx in (1..e.len()).step_by(2).rev() {
                        if matches!(e[idx], Expression::Add) && e.len() != 3 {
                            let a = e.remove(idx - 1);
                            let op = e.remove(idx - 1);
                            let b = e.remove(idx - 1);

                            e.insert(idx - 1, Expression::Group(vec![a, op, b]));
                        }
                    }
                    while e.len() != 3 {
                        let a = e.remove(0);
                        let op = e.remove(0);
                        let b = e.remove(0);

                        assert!(matches!(op, Expression::Add | Expression::Mul));

                        e.insert(0, Expression::Group(vec![a, op, b]));
                    }
                }
                Expression::Num(_) => {}
                Expression::Add | Expression::Mul => unreachable!(),
            }
        }
        Ok(self
            .expressions
            .iter_mut()
            .fold(0, |acc, expr| {
                group(expr);
                acc + expr.eval()
            })
            .into())
    }
}
