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

    let (w, h) = lines[0].split_once('x').unwrap();
    let w: usize = w.parse().unwrap();
    let h: usize = h.parse().unwrap();
    let data: Vec<char> = lines[1].chars().collect();
    assert!(data.len() % (w * h) == 0);
    let mut min_zero_count = usize::MAX;
    let mut ans = 0;
    for layer in data.chunks(w * h) {
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut two_count = 0;
        layer.iter().for_each(|&c| {
            if c == '0' {
                zero_count += 1;
            }
            if c == '1' {
                one_count += 1;
            }
            if c == '2' {
                two_count += 1;
            }
        });

        if zero_count < min_zero_count {
            ans = one_count * two_count;
            min_zero_count = zero_count;
        }
    }

    println!("ans: {ans}");
}
