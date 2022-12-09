#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut head = (0isize, 0isize);
    let mut tail = (0isize, 0isize);

    let mut tail_pos = BTreeSet::new();

    for line in &lines {
        if line.is_empty() {
            continue;
        }

        let (dir, cnt) = line.split_once(' ').unwrap();
        let cnt: usize = cnt.parse().unwrap();

        for _ in 0..cnt {
            match dir {
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => unreachable!(),
            }

            let x_diff = head.0 - tail.0;
            let y_diff = head.1 - tail.1;

            if y_diff.abs() > 1 {
                tail.1 += if y_diff > 0 { 1 } else { -1 };
                if x_diff.abs() == 1 {
                    tail.0 += if x_diff > 0 { 1 } else { -1 };
                }
            } else if x_diff.abs() > 1 {
                tail.0 += if x_diff > 0 { 1 } else { -1 };
                if y_diff.abs() == 1 {
                    tail.1 += if y_diff > 0 { 1 } else { -1 };
                }
            }

            tail_pos.insert(tail);
        }
    }

    println!("ans: {}", tail_pos.len());
}
