#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

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

pub struct Day11 {
    monkeys: Vec<Monkey>,
}

impl Day11 {
    pub fn new() -> Self {
        Self {
            monkeys: Vec::new(),
        }
    }

    fn rounds<const ROUNDS: usize, const PART1: bool>(&mut self) -> usize {
        let max = self.monkeys.iter().fold(1, |acc, monkey| acc * monkey.test);
        for _ in 0..ROUNDS {
            for idx in 0..self.monkeys.len() {
                self.monkeys[idx].inspected += self.monkeys[idx].items.len();
                while !self.monkeys[idx].items.is_empty() {
                    let mut worry = self.monkeys[idx].items.remove(0);
                    match self.monkeys[idx].op {
                        Op::Add(n) => worry += n,
                        Op::Mul(n) => worry *= n,
                        Op::Square => worry *= worry,
                        Op::None => unreachable!(),
                    }
                    if PART1 {
                        worry /= 3;
                    }
                    let throw_to = if worry % self.monkeys[idx].test == 0 {
                        self.monkeys[idx].if_true
                    } else {
                        self.monkeys[idx].if_false
                    };
                    worry %= max;
                    self.monkeys[throw_to].items.push(worry);
                }
            }
        }

        self.monkeys.sort_by_key(|m| m.inspected);
        self.monkeys
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, monkey| acc * monkey.inspected)
    }
}

impl Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for lines in lines.chunks(7) {
            let mut monkey = Monkey::default();
            // Parse Starting
            for item in lines[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
            {
                monkey.items.push(item.parse()?);
            }
            // Parse Operation
            match lines[2]
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .split_once(' ')
                .unwrap()
            {
                ("+", num) => monkey.op = Op::Add(num.parse()?),
                ("*", "old") => monkey.op = Op::Square,
                ("*", num) => monkey.op = Op::Mul(num.parse()?),
                _ => unreachable!(),
            }
            // Parse Test
            monkey.test = lines[3]
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()?;
            // Parse True
            monkey.if_true = lines[4]
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()?;
            // Parse False
            monkey.if_false = lines[5]
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()?;
            // Save monkey
            self.monkeys.push(monkey);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.rounds::<20, true>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.rounds::<10000, false>().into())
    }
}
