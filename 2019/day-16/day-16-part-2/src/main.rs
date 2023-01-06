const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample-2.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};

fn phase(skip: usize, signal: &[isize], new_signal: &mut [isize]) {
    let mut num = 0;
    for i in (skip..signal.len()).rev() {
        num += signal[i];
        new_signal[i] = num.abs() % 10;
    }
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

        let mut original_signal: Vec<isize> = Vec::new();
        for c in line.chars() {
            let num = (c as u32 - '0' as u32) as isize;
            original_signal.push(num);
        }

        println!("{line}");
        const REPEAT: usize = 10000;
        let mut signal = Vec::with_capacity(original_signal.len() * REPEAT);
        for _ in 0..REPEAT {
            signal.extend_from_slice(&original_signal);
        }
        let mut new_signal = signal.clone();
        let offset = &line[0..7];
        let offset: usize = offset.parse().unwrap();
        for _ in 1..=100 {
            phase(offset, &signal, &mut new_signal);
            std::mem::swap(&mut signal, &mut new_signal);
        }
        print!("offset: {offset}  ans: ");
        for val in &signal[offset..offset + 8] {
            print!("{val}");
        }
        println!();
    }
}
