#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug)]
enum Op {
    #[default]
    None,
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Default, Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut monkey = Monkey::default();
    let mut monkeys = Vec::new();
    let mut max = 1;
    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue;
        }
        let line = line.trim();
        let parts: Vec<&str> = line.split(' ').collect();

        match parts[0] {
            "Monkey" => {
                max *= monkey.test;
                monkeys.push(monkey);
                monkey = Monkey::default();
            }
            "Starting" => {
                for item in &parts[2..] {
                    let item = item.trim_end_matches(',').parse().unwrap();
                    monkey.items.push(item);
                }
            }
            "Operation:" if parts[4] == "+" => monkey.op = Op::Add(parts[5].parse().unwrap()),
            "Operation:" if parts[4] == "*" && parts[5] == "old" => monkey.op = Op::Square,
            "Operation:" if parts[4] == "*" => monkey.op = Op::Mul(parts[5].parse().unwrap()),
            "Test:" => monkey.test = parts[3].parse().unwrap(),
            "If" if parts[1] == "true:" => {
                monkey.if_true = parts[5].parse().unwrap();
            }
            "If" if parts[1] == "false:" => {
                monkey.if_false = parts[5].parse().unwrap();
            }
            _ => unreachable!(),
        }
    }
    max *= monkey.test;
    monkeys.push(monkey);

    println!("max:{max}");

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            monkeys[idx].inspected += monkeys[idx].items.len();
            while !monkeys[idx].items.is_empty() {
                let mut worry = monkeys[idx].items.remove(0);
                match monkeys[idx].op {
                    Op::Add(n) => worry += n,
                    Op::Mul(n) => worry *= n,
                    Op::Square => worry *= worry,
                    Op::None => unreachable!(),
                }
                let throw_to = if worry % monkeys[idx].test == 0 {
                    monkeys[idx].if_true
                } else {
                    monkeys[idx].if_false
                };
                worry %= max;
                monkeys[throw_to].items.push(worry);
            }
        }
    }

    println!("Monkeys:\n{monkeys:#?}");

    monkeys.sort_by_key(|m| m.inspected);
    monkeys.reverse();

    println!("ans: {}", monkeys[0].inspected * monkeys[1].inspected);
}
