#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day04 {
    cards: Vec<Card>,
}

impl Day04 {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
}

impl Runner for Day04 {
    fn parse(&mut self, path: &str) -> Result<(), Error> {
        for line in Lines::from_path(path, LinesOpt::TRIM)?.iter() {
            let (card, numbers) = line.split_once(":").expect("Could not split card");
            let (_, card) = card.split_once(' ').expect("Could not split card");
            let card = card.trim().parse().expect("Could not parse card number");
            let (winning, picked) = numbers.split_once("|").expect("Could not split numbers");
            let winning: Vec<usize> = winning
                .trim()
                .split(' ')
                .filter_map(|num| {
                    if !num.is_empty() {
                        Some(num.parse().expect("Could not parse number"))
                    } else {
                        None
                    }
                })
                .collect();
            let picked: Vec<usize> = picked
                .trim()
                .split(' ')
                .filter_map(|num| {
                    if !num.is_empty() {
                        Some(num.parse().expect("Could not parse number"))
                    } else {
                        None
                    }
                })
                .collect();
            self.cards.push(Card {
                num: card,
                copies: 1,
                winning,
                picked,
            });
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .cards
            .iter()
            .map(|c| {
                let matched = c.numbers_matched();
                if matched == 0 {
                    0
                } else {
                    1 << (matched - 1)
                }
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for i in 0..self.cards.len() {
            let matches = self.cards[i].numbers_matched();
            for j in i + 1..=i + matches {
                self.cards[j].copies += self.cards[i].copies;
            }
        }
        Ok(self.cards.iter().map(|c| c.copies).sum::<usize>().into())
    }
}

pub struct Card {
    num: usize,
    copies: usize,
    winning: Vec<usize>,
    picked: Vec<usize>,
}

impl Card {
    fn numbers_matched(&self) -> usize {
        let matched: usize = self
            .picked
            .iter()
            .map(|n| if self.winning.contains(n) { 1 } else { 0 })
            .sum();
        println!("Card {} matched {matched} numbers", self.num);
        matched
    }
}
