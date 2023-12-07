use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day07 {
    hands: Vec<Hand>,
}

impl Day07 {
    pub fn new() -> Self {
        Self { hands: Vec::new() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    card_str: String,
    cards: Vec<u8>,
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn new(input: &str, part1: bool) -> Hand {
        let (cards_str, bid) = input.split_once(' ').unwrap();
        let bid: usize = bid.parse().unwrap();
        let high_card_adjust = if part1 { 0 } else { 1 };
        let cards: Vec<u8> = cards_str
            .chars()
            .map(|c| match c {
                'A' => 14 - high_card_adjust,
                'K' => 13 - high_card_adjust,
                'Q' => 12 - high_card_adjust,
                'J' => {
                    if part1 {
                        11
                    } else {
                        1
                    }
                }
                'T' => 10,
                '2'..='9' => c as u8 - b'0',
                _ => unreachable!("Unknown card '{c}'"),
            })
            .collect();
        assert_eq!(cards.len(), 5);

        let hand_type = if part1 {
            Self::part1_hand_type(&cards)
        } else {
            Self::part2_hand_type(&cards)
        };

        Self {
            card_str: cards_str.into(),
            cards,
            bid,
            hand_type,
        }
    }

    fn part1_hand_type(cards: &[u8]) -> HandType {
        let mut card_count: BTreeMap<u8, u8> = BTreeMap::new();
        for card in cards.iter() {
            *card_count.entry(*card).or_default() += 1;
        }
        let mut card_count_values: Vec<u8> = card_count.values().copied().collect();
        card_count_values.sort();
        match card_count.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => match *card_count_values.as_slice() {
                [1, 2, 2] => HandType::TwoPair,
                [1, 1, 3] => HandType::ThreeOfAKind,
                _ => unreachable!("What?  {cards:?}  {card_count:?} {card_count_values:?}"),
            },
            2 => match *card_count_values.as_slice() {
                [2, 3] => HandType::FullHouse,
                [1, 4] => HandType::FourOfAKind,
                _ => unreachable!("What?  {cards:?}  {card_count:?} {card_count_values:?}"),
            },
            1 => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }

    fn part2_hand_type(cards: &[u8]) -> HandType {
        let jokers_at: Vec<usize> = cards
            .iter()
            .enumerate()
            .flat_map(|(i, card)| if *card == 1 { Some(i) } else { None })
            .collect();

        match (jokers_at.len(), Self::part1_hand_type(cards)) {
            (1, HandType::HighCard) => HandType::OnePair,
            (1, HandType::OnePair) => HandType::ThreeOfAKind,
            (1, HandType::TwoPair) => HandType::FullHouse,
            (1, HandType::ThreeOfAKind) => HandType::FourOfAKind,
            (1, HandType::FourOfAKind) => HandType::FiveOfAKind,
            (2, HandType::OnePair) => HandType::ThreeOfAKind,
            (2, HandType::TwoPair) => HandType::FourOfAKind,
            (2, HandType::FullHouse) => HandType::FiveOfAKind,
            (3, HandType::ThreeOfAKind) => HandType::FourOfAKind,
            (3, HandType::FullHouse) => HandType::FiveOfAKind,
            (4, HandType::FourOfAKind) => HandType::FiveOfAKind,
            (_, x) => x,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(<Self as Ord>::cmp(self, other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match a.cmp(b) {
                        Ordering::Equal => continue,
                        x => return x,
                    }
                }
                panic!("tie?  {self:?} {other:?}");
            }
            x => x,
        }
    }
}

impl Runner for Day07 {
    fn parse(&mut self, path: &str, part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.hands.push(Hand::new(line, part1));
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.hands.sort();
        for hand in self.hands.iter() {
            println!("{hand:?}");
        }
        Ok(self
            .hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bid)
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.hands.sort();
        for hand in self.hands.iter() {
            println!("{hand:?}");
        }
        Ok(self
            .hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bid)
            .sum::<usize>()
            .into())
    }
}
