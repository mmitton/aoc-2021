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

    const LEN: usize = 14;

    for line in lines {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                let chars: Vec<char> = line.chars().collect();

                'search_loop: for i in 0..chars.len() - (LEN - 1) {
                    let start = &chars[i..i + LEN];
                    for j in 0..start.len() {
                        for k in 0..start.len() {
                            if j == k {
                                continue;
                            }
                            if start[j] == start[k] {
                                continue 'search_loop;
                            }
                        }
                    }

                    println!("start of packet!  {}", i + LEN);
                    break;
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
