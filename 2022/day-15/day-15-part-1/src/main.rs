const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    const ROW: isize = if cfg!(debug_assertions) { 10 } else { 2000000 };
    let mut seen: BTreeMap<isize, bool> = BTreeMap::new();

    for line in lines {
        let line = line.replace('=', " ").replace(',', "").replace(':', "");
        let parts: Vec<&str> = line.split(' ').collect();
        let sx: isize = parts[3].parse().unwrap();
        let sy: isize = parts[5].parse().unwrap();
        let bx: isize = parts[11].parse().unwrap();
        let by: isize = parts[13].parse().unwrap();

        if by == ROW {
            seen.insert(bx, true);
        }

        let d = (sx - bx).abs() + (sy - by).abs();
        if (sy - ROW).abs() > d {
            continue;
        }
        let dx = d - (sy - ROW).abs();
        for x in sx - dx..=sx + dx {
            if !seen.contains_key(&x) {
                seen.insert(x, false);
            }
        }
    }

    let ans: usize = seen.iter().map(|(_, v)| if !v { 1 } else { 0 }).sum();
    println!("ans: {ans:?}");
}
