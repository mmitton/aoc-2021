const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample-2.txt"
} else {
    "../input-2.txt"
};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Op {
    Reverse,
    Cut(isize),
    DealSkipN(usize),
}

fn process(cards: &Option<usize>, iters: &Option<usize>, card: &Option<usize>, ops: &Vec<Op>) {
    if cards.is_none() || iters.is_none() || card.is_none() || ops.is_empty() {
        return;
    }

    let cards = cards.unwrap();
    let mut card = card.unwrap();
    let iters = iters.unwrap();
    // let mut seen: BTreeMap<usize, usize> = BTreeMap::new();
    // seen.insert(card, 0);
    for iter in 0..iters {
        continue;
        for op in ops.iter().rev() {
            match op {
                Op::Reverse => {
                    card = cards - card - 1;
                }
                Op::Cut(n) => {
                    let n = if *n > 0 {
                        *n as usize
                    } else {
                        cards - n.unsigned_abs()
                    };
                    if card < cards - n {
                        card += n;
                    } else {
                        card = n - (cards - card);
                    }
                }
                Op::DealSkipN(n) => {
                    let mut i = 0;
                    let mut start = 0;
                    while card < i || (card - i) % n != 0 {
                        let num_this_iter = ((cards - i - 1) / n) + 1;
                        i = (i + (num_this_iter * n)) % cards;
                        start += num_this_iter;
                    }
                    let offset = (card - i) / n;
                    card = start + offset;
                }
            }
        }
        // if let Some(seen_at) = seen.get(&card) {
        //     println!("Cycle seen with card {card} from {seen_at} to {iter}");
        // } else {
        //     seen.insert(card, iter);
        //     // println!("{card} {iter}");
        // }
    }

    println!("ans: {card}");
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut cards: Option<usize> = None;
    let mut iters: Option<usize> = None;
    let mut card: Option<usize> = None;
    let mut ops: Vec<Op> = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            process(&cards, &iters, &card, &ops);
            cards = None;
            iters = None;
            card = None;
            ops.clear();
            continue;
        }

        if cards.is_none() {
            cards = Some(line.parse().unwrap());
        } else if iters.is_none() {
            iters = Some(line.parse().unwrap());
        } else if card.is_none() {
            card = Some(line.parse().unwrap());
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
    process(&cards, &iters, &card, &ops);
}
