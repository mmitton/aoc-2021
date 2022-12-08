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

    let lines: Vec<String> = lines.flatten().collect();

    let mut map: Vec<Vec<(isize, bool)>> = Vec::new();
    for line in &lines {
        let mut row = Vec::new();
        for c in line.chars() {
            let height: isize = c as isize - '0' as isize;
            row.push((height, false));
        }
        map.push(row);
    }

    for row in 0..map[0].len() {
        let mut max = -1;
        for col in 0..map.len() {
            if map[row][col].0 > max {
                max = map[row][col].0;
                map[row][col].1 = true;
            }
        }

        let mut max = -1;
        for col in (0..map.len()).rev() {
            if map[row][col].0 > max {
                max = map[row][col].0;
                map[row][col].1 = true;
            }
        }
    }
    for col in 0..map.len() {
        let mut max = -1;
        for row in 0..map[0].len() {
            if map[row][col].0 > max {
                max = map[row][col].0;
                map[row][col].1 = true;
            }
        }

        let mut max = -1;
        for row in (0..map[0].len()).rev() {
            if map[row][col].0 > max {
                max = map[row][col].0;
                map[row][col].1 = true;
            }
        }
    }

    let mut ans = 0;
    for row in 0..map[0].len() {
        for col in 0..map.len() {
            if map[row][col].1 {
                println!("Visible {col},{row}");
                ans += 1;
            }
        }
    }

    println!("ans: {ans}");
}
