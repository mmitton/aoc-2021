use std::ops::RangeInclusive;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day16 {
    rules: Vec<Rule>,
    tickets: Vec<Vec<Num>>,
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: [RangeInclusive<usize>; 2],
    matched: usize,
    matches: Vec<usize>,
}

#[derive(Copy, Clone, Debug)]
struct Num {
    num: usize,
    matches: u32,
}

impl Day16 {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            tickets: Vec::new(),
        }
    }

    fn filter_tickets(&mut self) -> usize {
        let mut sum = 0;
        'tickets: for ticket in self.tickets.iter_mut() {
            for num in ticket.iter_mut() {
                for (idx, rule) in self.rules.iter().enumerate() {
                    if rule.ranges[0].contains(&num.num) || rule.ranges[1].contains(&num.num) {
                        num.matches |= 1 << idx;
                    }
                }
                if num.matches == 0 {
                    sum += num.num;
                    ticket.clear();
                    continue 'tickets;
                }
            }
        }

        self.tickets.retain(|t| !t.is_empty());

        sum
    }

    fn match_numbers(&mut self) {
        for (i, r) in self.rules.iter_mut().enumerate() {
            for n in 0..self.tickets[0].len() {
                if self
                    .tickets
                    .iter()
                    .filter(|t| t[n].matches & 1 << i != 0)
                    .count()
                    == self.tickets.len()
                {
                    r.matches.push(n);
                }
            }
        }

        loop {
            let mut changed = false;
            for i in 0..self.rules.len() {
                if self.rules[i].matches.len() == 1 {
                    let matched = self.rules[i].matches[0];
                    for r in self.rules.iter_mut() {
                        r.matches.retain(|m| *m != matched);
                    }
                    self.rules[i].matched = matched;
                    changed = true;
                }
            }

            if !changed {
                break;
            }
        }
    }
}

impl Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let (name, ranges) = line.split_once(": ").unwrap();
            let (r1, r2) = ranges.split_once(" or ").unwrap();

            fn parse(r: &str) -> RangeInclusive<usize> {
                let (lo, hi) = r.split_once('-').unwrap();
                let lo: usize = lo.parse().unwrap();
                let hi: usize = hi.parse().unwrap();
                lo..=hi
            }

            let r1 = parse(r1);
            let r2 = parse(r2);

            self.rules.push(Rule {
                name: name.into(),
                ranges: [r1, r2],
                matched: usize::MAX,
                matches: Vec::new(),
            });
        }

        for line in lines {
            if line.is_empty() || line.contains(':') {
                continue;
            }

            self.tickets.push(
                line.split(',')
                    .map(|n| Num {
                        num: n.parse().unwrap(),
                        matches: 0,
                    })
                    .collect(),
            );
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.filter_tickets().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.filter_tickets();
        self.match_numbers();

        let prefix = if self.rules.len() == 3 {
            "seat"
        } else {
            "departure"
        };

        Ok(self
            .rules
            .iter()
            .filter_map(|r| {
                if r.name.starts_with(prefix) {
                    Some(self.tickets[0][r.matched].num)
                } else {
                    None
                }
            })
            .product::<usize>()
            .into())
    }
}
