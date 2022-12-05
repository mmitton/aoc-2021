#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Pile(VecDeque<char>);

impl Pile {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn push_back(&mut self, c: char) {
        self.0.push_back(c);
    }

    fn take(&mut self, num: usize) -> Vec<char> {
        let mut chars = Vec::new();

        for _ in 0..num {
            chars.push(self.0.pop_front().unwrap());
        }

        chars
    }

    fn put(&mut self, chars: &[char]) {
        for c in chars.iter().rev() {
            self.0.push_front(*c);
        }
    }

    fn peek_top(&self) -> Option<char> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0[0])
        }
    }
}

fn main() {
    let lines = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines()
    };

    let mut piles = Vec::new();
    let mut state = 0;
    for line in lines {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    state += 1;
                } else if state == 0 {
                    let chars: Vec<char> = line.chars().collect();
                    for (pos, c) in chars.iter().skip(1).step_by(4).enumerate() {
                        while piles.len() <= pos {
                            piles.push(Pile::new());
                        }
                        if *c >= 'A' && *c <= 'Z' {
                            piles[pos].push_back(*c);
                        }
                    }
                } else if state == 1 {
                    let parts: Vec<&str> = line.split(' ').collect();
                    let num: usize = parts[1].parse().unwrap();
                    let pile_from: usize = parts[3].parse().unwrap();
                    let pile_to: usize = parts[5].parse().unwrap();

                    let take = piles[pile_from - 1].take(num);
                    piles[pile_to - 1].put(&take);
                } else {
                    unreachable!();
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    print!("answer: ");
    for pile in &piles {
        print!("{}", pile.peek_top().unwrap());
    }
    println!();
}
