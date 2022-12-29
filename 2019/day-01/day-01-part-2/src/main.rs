const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut ans = 0;
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }

        let mut mass: isize = line.parse().unwrap();
        let mut total = 0;
        loop {
            mass = (mass / 3) - 2;
            if mass < 0 {
                break;
            }
            total += mass;
        }
        println!("{line} : {total}");
        ans += total;
    }

    println!("ans: {ans}");
}
