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

    let mut total = 0;
    for line in lines {
        match line {
            Ok(line) => {
                let line = line.trim();
                assert!(line.len() % 2 == 0);
                assert!(!line.is_empty());

                let split = line.len() / 2;
                let comp1: Vec<char> = line[..split].chars().collect();
                let comp2: Vec<char> = line[split..].chars().collect();

                for c in &comp1 {
                    if comp2.contains(c) {
                        println!("in both: {}", c);
                        total += match c {
                            'a'..='z' => (*c as u32 - 'a' as u32) + 1,
                            'A'..='Z' => (*c as u32 - 'A' as u32) + 27,
                            _ => unreachable!(),
                        };
                        break;
                    }
                }

                println!("comp1:{:?}", comp1);
                println!("comp2:{:?}", comp2);
            }
            Err(e) => panic!("{}", e),
        }
    }

    println!("total: {}", total);
}
