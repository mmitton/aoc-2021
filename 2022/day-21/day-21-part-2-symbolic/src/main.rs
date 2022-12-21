const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use core::fmt;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Monkey {
    Num(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug, Clone)]
enum Op {
    Num(isize),
    Human,
    Add(Box<Op>, Box<Op>),
    Sub(Box<Op>, Box<Op>),
    Mul(Box<Op>, Box<Op>),
    Div(Box<Op>, Box<Op>),
}

impl Op {
    fn simplify(&self) -> Self {
        match self {
            Self::Num(_) | Self::Human => self.clone(),
            Self::Add(a, b) => {
                let mut a = a.simplify();
                let mut b = b.simplify();
                if !matches!(a, Op::Num(_)) && matches!(b, Op::Num(_)) {
                    // Swap them
                    std::mem::swap(&mut a, &mut b);
                }
                match (&a, &b) {
                    (Op::Num(a), Op::Num(b)) => Op::Num(a + b),
                    (Op::Num(n), Op::Add(a, b)) => match (a.as_ref(), b.as_ref()) {
                        (Op::Num(a), _) => Op::Add(Box::new(Op::Num(n + a)), b.clone()).simplify(),
                        (_, Op::Num(b)) => Op::Add(Box::new(Op::Num(n + b)), a.clone()).simplify(),
                        _ => Self::Add(
                            Box::new(Op::Num(*n)),
                            Box::new(Op::Add(a.clone(), b.clone())),
                        ),
                    },
                    (Op::Num(n), Op::Sub(a, b)) => match (a.as_ref(), b.as_ref()) {
                        (Op::Num(a), _) => Op::Add(Box::new(Op::Num(n - a)), b.clone()).simplify(),
                        (_, Op::Num(b)) => Op::Add(Box::new(Op::Num(n - b)), a.clone()).simplify(),
                        _ => Self::Add(
                            Box::new(Op::Num(*n)),
                            Box::new(Op::Sub(a.clone(), b.clone())),
                        ),
                    },
                    _ => Self::Add(Box::new(a), Box::new(b)),
                }
            }
            Self::Sub(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    // (a - b)
                    (Op::Num(a), Op::Num(b)) => Op::Num(a - b),
                    // n - (a + b)
                    (Op::Num(n), Op::Add(a, b)) => match (a.as_ref(), b.as_ref()) {
                        // (n - a) + b
                        (Op::Num(a), _) => Op::Add(Box::new(Op::Num(n - a)), b.clone()).simplify(),
                        // (n - b) + a
                        (_, Op::Num(b)) => Op::Add(Box::new(Op::Num(n - b)), a.clone()).simplify(),
                        _ => Self::Sub(
                            Box::new(Op::Num(*n)),
                            Box::new(Op::Add(a.clone(), b.clone())),
                        ),
                    },
                    // n - (a - b)
                    (Op::Num(n), Op::Sub(a, b)) => match (a.as_ref(), b.as_ref()) {
                        // (n - a) - b
                        (Op::Num(a), _) => Op::Sub(Box::new(Op::Num(n - a)), b.clone()).simplify(),
                        // (n - b) - a
                        (_, Op::Num(b)) => Op::Sub(Box::new(Op::Num(n - b)), a.clone()).simplify(),
                        _ => Self::Sub(
                            Box::new(Op::Num(*n)),
                            Box::new(Op::Sub(a.clone(), b.clone())),
                        ),
                    },
                    // (a + b) - n
                    (Op::Add(a, b), Op::Num(n)) => match (a.as_ref(), b.as_ref()) {
                        // (a - n) + b
                        (Op::Num(a), _) => Op::Add(Box::new(Op::Num(a - n)), b.clone()).simplify(),
                        // (b - n) + a
                        (_, Op::Num(b)) => Op::Add(Box::new(Op::Num(b - n)), a.clone()).simplify(),
                        _ => Self::Sub(
                            Box::new(Op::Sub(a.clone(), b.clone())),
                            Box::new(Op::Num(*n)),
                        ),
                    },
                    // (a - b) - n
                    (Op::Sub(a, b), Op::Num(n)) => match (a.as_ref(), b.as_ref()) {
                        // (a - n) - b
                        (Op::Num(a), _) => Op::Sub(Box::new(Op::Num(a - n)), b.clone()).simplify(),
                        // a - (b - n)
                        (_, Op::Num(b)) => Op::Sub(a.clone(), Box::new(Op::Num(b + n))).simplify(),
                        _ => Self::Sub(
                            Box::new(Op::Sub(a.clone(), b.clone())),
                            Box::new(Op::Num(*n)),
                        ),
                    },
                    _ => Self::Sub(Box::new(a), Box::new(b)),
                }
            }
            Self::Mul(a, b) => {
                let mut a = a.simplify();
                let mut b = b.simplify();
                if !matches!(a, Op::Num(_)) && matches!(b, Op::Num(_)) {
                    // Swap them
                    std::mem::swap(&mut a, &mut b);
                }
                match (&a, &b) {
                    (Op::Num(a), Op::Num(b)) => Op::Num(a * b),
                    (Op::Num(a), _) if *a == 1 => b.clone(),
                    (_, Op::Num(b)) if *b == 1 => a.clone(),
                    (Op::Num(n), Op::Add(a, b)) => {
                        let a = Op::Mul(Box::new(Op::Num(*n)), a.clone());
                        let b = Op::Mul(Box::new(Op::Num(*n)), b.clone());
                        let a = a.simplify();
                        let b = b.simplify();
                        let res = Op::Add(Box::new(a), Box::new(b));
                        res.simplify()
                    }
                    (Op::Num(n), Op::Sub(a, b)) => {
                        let a = Op::Mul(Box::new(Op::Num(*n)), a.clone());
                        let b = Op::Mul(Box::new(Op::Num(*n)), b.clone());
                        let a = a.simplify();
                        let b = b.simplify();
                        let res = Op::Sub(Box::new(a), Box::new(b));
                        res.simplify()
                    }
                    (Op::Num(n), Op::Mul(a, b)) => match (a.as_ref(), b.as_ref()) {
                        (Op::Num(a), _) => Op::Mul(Box::new(Op::Num(a * n)), b.clone()).simplify(),
                        (_, Op::Num(b)) => Op::Mul(Box::new(Op::Num(b * n)), a.clone()).simplify(),
                        _ => Op::Mul(
                            Box::new(Op::Num(*n)),
                            Box::new(Op::Mul(a.clone(), b.clone())),
                        ),
                    },
                    _ => Self::Mul(Box::new(a), Box::new(b)),
                }
            }
            Self::Div(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (Op::Num(a), Op::Num(b)) => Op::Num(a / b),
                    (Op::Add(a, b), Op::Num(n)) => match (a.as_ref(), b.as_ref()) {
                        (Op::Num(a), _) if *a % *n == 0 => Op::Add(
                            Box::new(Op::Num(a / n)),
                            Box::new(Op::Div(b.clone(), Box::new(Op::Num(*n)))),
                        )
                        .simplify(),
                        (_, Op::Num(b)) if *b % *n == 0 => Op::Add(
                            Box::new(Op::Num(b / n)),
                            Box::new(Op::Div(a.clone(), Box::new(Op::Num(*n)))),
                        )
                        .simplify(),
                        _ => Self::Div(
                            Box::new(Op::Add(a.clone(), b.clone())),
                            Box::new(Op::Num(*n)),
                        ),
                    },
                    (Op::Mul(a, b), Op::Num(n)) => match (a.as_ref(), b.as_ref()) {
                        (Op::Num(a), _) if *a % *n == 0 => {
                            Op::Mul(Box::new(Op::Num(a / n)), b.clone()).simplify()
                        }
                        (_, Op::Num(b)) if *b % *n == 0 => {
                            Op::Mul(Box::new(Op::Num(b / n)), a.clone()).simplify()
                        }
                        _ => Self::Div(
                            Box::new(Op::Mul(a.clone(), b.clone())),
                            Box::new(Op::Num(*n)),
                        ),
                    },
                    // (a / b) / n
                    (Op::Div(a, b), Op::Num(n)) => match (a.as_ref(), b.as_ref()) {
                        // (a / n) / b
                        (Op::Num(a), _) => Op::Div(Box::new(Op::Num(a / n)), b.clone()).simplify(),
                        // a / (b * n)
                        (_, Op::Num(b)) => Op::Div(a.clone(), Box::new(Op::Num(b * n))).simplify(),
                        _ => Self::Div(
                            Box::new(Op::Div(a.clone(), b.clone())),
                            Box::new(Op::Num(*n)),
                        ),
                    },
                    _ => Self::Div(Box::new(a), Box::new(b)),
                }
            }
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::Human => write!(f, "humn"),
            Self::Add(a, b) => write!(f, "({a} + {b})"),
            Self::Sub(a, b) => write!(f, "({a} - {b})"),
            Self::Mul(a, b) => write!(f, "({a} * {b})"),
            Self::Div(a, b) => write!(f, "({a} / {b})"),
        }
    }
}

fn solve(left: Op, right: Op) -> (Op, Op) {
    let mut left = left;
    let mut right = right;

    loop {
        if !matches!(left, Op::Num(_)) && matches!(right, Op::Num(_)) {
            // Swap them
            std::mem::swap(&mut left, &mut right);
        }

        if let Op::Num(n) = left {
            match right {
                Op::Num(_) | Op::Human => break,
                Op::Add(a, b) => match (a.as_ref(), b.as_ref()) {
                    (Op::Num(a), _) => {
                        left = Op::Num(n - a);
                        right = *b.clone();
                    }
                    (_, Op::Num(b)) => {
                        left = Op::Num(n - b);
                        right = *a.clone();
                    }
                    _ => unreachable!(),
                },
                Op::Sub(a, b) => match (a.as_ref(), b.as_ref()) {
                    (Op::Num(a), _) => {
                        left = Op::Num(-(n - a));
                        right = *b.clone();
                    }
                    (_, Op::Num(b)) => {
                        left = Op::Num(n + b);
                        right = *a.clone();
                    }
                    _ => unreachable!(),
                },
                Op::Mul(a, b) => match (a.as_ref(), b.as_ref()) {
                    // a * b = n  b = n / a
                    (Op::Num(a), _) => {
                        left = Op::Num(n / a);
                        right = *b.clone();
                    }
                    // a * b = n  a = n / b
                    (_, Op::Num(b)) => {
                        left = Op::Num(n / b);
                        right = *a.clone();
                    }
                    _ => unreachable!(),
                },
                Op::Div(a, b) => match (a.as_ref(), b.as_ref()) {
                    // a / b = n  b = a / n
                    (Op::Num(a), _) => {
                        left = Op::Num(a / n);
                        right = *b.clone();
                    }
                    // a / b = n  a = b * n
                    (_, Op::Num(b)) => {
                        left = Op::Num(n * b);
                        right = *a.clone();
                    }
                    _ => unreachable!(),
                },
            }
        } else {
            unreachable!();
        }
    }

    (left, right)
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut monkeys: BTreeMap<String, Monkey> = BTreeMap::new();
    let mut ops = BTreeMap::new();
    let mut root = None;
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let name = line[0..4].to_string();
        let op = &line[6..];
        match name.as_str() {
            "root" => {
                let (a, b) = op.split_once(" + ").unwrap();
                root = Some((a.to_owned(), b.to_owned()));
            }
            "humn" => {
                ops.insert("humn".to_string(), Op::Human);
            }
            _ => {
                if let Some((a, b)) = op.split_once(" + ") {
                    monkeys.insert(name, Monkey::Add(a.to_string(), b.to_string()));
                } else if let Some((a, b)) = op.split_once(" - ") {
                    monkeys.insert(name, Monkey::Sub(a.to_string(), b.to_string()));
                } else if let Some((a, b)) = op.split_once(" * ") {
                    monkeys.insert(name, Monkey::Mul(a.to_string(), b.to_string()));
                } else if let Some((a, b)) = op.split_once(" / ") {
                    monkeys.insert(name, Monkey::Div(a.to_string(), b.to_string()));
                } else {
                    monkeys.insert(name, Monkey::Num(op.parse().unwrap()));
                }
            }
        }
    }

    let mut work_to_do = true;
    while work_to_do {
        work_to_do = false;
        for (name, monkey) in monkeys.iter() {
            if ops.contains_key(name) {
                continue;
            }
            match monkey {
                Monkey::Num(num) => {
                    work_to_do = true;
                    ops.insert(name.clone(), Op::Num(*num));
                }
                Monkey::Add(a, b) => {
                    if let (Some(a), Some(b)) = (ops.get(a), ops.get(b)) {
                        work_to_do = true;
                        ops.insert(
                            name.clone(),
                            Op::Add(Box::new(a.clone()), Box::new(b.clone())),
                        );
                    }
                }
                Monkey::Sub(a, b) => {
                    if let (Some(a), Some(b)) = (ops.get(a), ops.get(b)) {
                        work_to_do = true;
                        ops.insert(
                            name.clone(),
                            Op::Sub(Box::new(a.clone()), Box::new(b.clone())),
                        );
                    }
                }
                Monkey::Mul(a, b) => {
                    if let (Some(a), Some(b)) = (ops.get(a), ops.get(b)) {
                        work_to_do = true;
                        ops.insert(
                            name.clone(),
                            Op::Mul(Box::new(a.clone()), Box::new(b.clone())),
                        );
                    }
                }
                Monkey::Div(a, b) => {
                    if let (Some(a), Some(b)) = (ops.get(a), ops.get(b)) {
                        work_to_do = true;
                        ops.insert(
                            name.clone(),
                            Op::Div(Box::new(a.clone()), Box::new(b.clone())),
                        );
                    }
                }
            }
        }
    }

    if let Some((a, b)) = root {
        let a = ops.get(&a).unwrap();
        let b = ops.get(&b).unwrap();

        let a = a.simplify();
        let b = b.simplify();

        let (a, b) = solve(a, b);
        println!("{a} = {b}");
    }
}
