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

    let mut cals: Vec<usize> = Vec::new();
    let mut cal = None;
    for line in lines {
        match line {
            Ok(line) => {
                if line.trim() == "" {
                    if let Some(elf_cal) = cal.take() {
                        cals.push(elf_cal);
                        cal = None;
                    }
                } else {
                    let item_cal = line.parse::<usize>().expect("Cannot parse integer");
                    if cal.is_none() {
                        cal = Some(item_cal);
                    } else {
                        *cal.as_mut().unwrap() += item_cal;
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
    if let Some(elf_cal) = cal.take() {
        cals.push(elf_cal);
    }

    cals.sort();
    cals.reverse();
    println!("cals: {:?}", cals);

    println!("Max: {}", cals[0]);
}
