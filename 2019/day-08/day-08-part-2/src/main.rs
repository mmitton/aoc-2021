const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample-2.txt"
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
    let mut image = vec![vec!['.'; w]; h];
    assert!(data.len() % (w * h) == 0);
    for layer in data.chunks(w * h) {
        for (idx, &c) in layer.iter().enumerate() {
            let x = idx % w;
            let y = idx / w;
            if image[y][x] == '.' && c != '2' {
                match c {
                    '0' => image[y][x] = ' ',
                    '1' => image[y][x] = '#',
                    _ => unreachable!(),
                }
            }
        }
    }

    for line in &image {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}
