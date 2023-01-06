const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};

fn phase(signal: &[isize]) -> Vec<isize> {
    let mut new_signal = Vec::with_capacity(signal.len());

    for i in 0..signal.len() {
        let mut val = 0;

        let mut add = true;
        let mut j = i;
        while j < signal.len() {
            let mut chunk = i + 1;
            while j < signal.len() && chunk > 0 {
                if add {
                    val += signal[j];
                } else {
                    val -= signal[j];
                }
                j += 1;
                chunk -= 1;
            }
            add = !add;
            j += i + 1;
        }

        new_signal.push(val.abs() % 10);
    }

    new_signal
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }

        let mut signal: Vec<isize> = Vec::new();
        for c in line.chars() {
            let num = (c as u32 - '0' as u32) as isize;
            signal.push(num);
        }

        println!("{line}");
        for _ in 1..=100 {
            signal = phase(&signal);
        }
        print!("ans: ");
        for val in &signal[0..8] {
            print!("{val}");
        }
        println!();
    }
}
