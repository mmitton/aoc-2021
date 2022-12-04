#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeBounds;

fn main() {
    let lines = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines()
    };

    let mut total = 0;
    for line in lines {
        match line {
            Ok(line) => {
                let (elf1, elf2) = line.split_once(',').unwrap();

                macro_rules! range {
                    ($range:expr) => {{
                        let (lo, hi) = $range.split_once('-').unwrap();
                        let lo: usize = lo.parse().unwrap();
                        let hi: usize = hi.parse().unwrap();
                        lo..=hi
                    }};
                }
                let elf1 = range!(elf1);
                let elf2 = range!(elf2);
                if elf1.contains(elf2.start()) || elf1.contains(elf2.end()) {
                    println!("elf1 contains elf2  {:?} {:?}", elf1, elf2);
                    total += 1;
                } else if elf2.contains(elf1.start()) || elf2.contains(elf1.end()) {
                    println!("elf2 contains elf1  {:?} {:?}", elf1, elf2);
                    total += 1;
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    println!("total: {}", total);
}
