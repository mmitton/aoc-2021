#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines()
    };

    let mut bags = Vec::new();
    for line in lines {
        match line {
            Ok(line) => {
                let line = line.trim();
                assert!(line.len() % 2 == 0);
                assert!(!line.is_empty());

                bags.push(line.to_string());
            }
            Err(e) => panic!("{}", e),
        }
    }

    let mut total = 0;
    for group in bags.chunks(3) {
        println!("{:?}", group);
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                total += match c {
                    'a'..='z' => (c as u32 - 'a' as u32) + 1,
                    'A'..='Z' => (c as u32 - 'A' as u32) + 27,
                    _ => unreachable!(),
                };
                break;
            }
        }
    }

    println!("total: {}", total);
}
