const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Op {
    Reverse,
    Cut(isize),
    DealSkipN(usize),
}

fn process(cards: &Option<usize>, ops: &Vec<Op>) {
    if cards.is_none() || ops.is_empty() {
        return;
    }

    let cards = cards.unwrap();
    let mut deck = Vec::with_capacity(cards);
    for i in 0..cards {
        deck.push(i);
    }
    let mut temp_deck = deck.clone();
    for op in ops.iter() {
        match op {
            Op::Reverse => {
                deck.reverse();
            }
            Op::Cut(n) => {
                let n = if *n > 0 {
                    *n as usize
                } else {
                    cards - n.unsigned_abs()
                };
                temp_deck.clear();
                temp_deck.extend_from_slice(&deck[n..]);
                temp_deck.extend_from_slice(&deck[..n]);
                std::mem::swap(&mut deck, &mut temp_deck);
            }
            Op::DealSkipN(n) => {
                for card in temp_deck.iter_mut() {
                    *card = !0;
                }
                let mut pos = 0;
                for card in deck.iter() {
                    assert!(temp_deck[pos % cards] == !0);
                    temp_deck[pos % cards] = *card;
                    pos += n;
                }
                std::mem::swap(&mut deck, &mut temp_deck);
            }
        }
    }

    let mut pos = vec![!0usize; cards];
    for (idx, card) in deck.iter().enumerate() {
        assert!(pos[*card] == !0);
        pos[*card] = idx;
    }

    if cfg!(debug_assertions) {
        print!("Result: ");
        for (idx, card) in deck.iter().enumerate() {
            if idx == 0 {
                print!("{card}");
            } else {
                print!(", {card}");
            }
        }
        println!();
    } else {
        println!("ans: {}", pos[2019]);
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut cards: Option<usize> = None;
    let mut ops: Vec<Op> = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            process(&cards, &ops);
            cards = None;
            ops.clear();
            continue;
        }

        if cards.is_none() {
            cards = Some(line.parse().unwrap());
        } else if line.as_str() == "deal into new stack" {
            ops.push(Op::Reverse)
        } else if let Some(num) = line.strip_prefix("cut ") {
            ops.push(Op::Cut(num.parse().unwrap()));
        } else if let Some(num) = line.strip_prefix("deal with increment ") {
            ops.push(Op::DealSkipN(num.parse().unwrap()));
        } else {
            unreachable!();
        }
    }
    process(&cards, &ops);
}
