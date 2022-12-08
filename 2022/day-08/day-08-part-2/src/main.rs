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

    let mut map: Vec<Vec<(usize, usize)>> = Vec::new();
    for line in &lines {
        let mut row = Vec::new();
        for c in line.chars() {
            let height: usize = c as usize - '0' as usize;
            row.push((height, 0));
        }
        map.push(row);
    }

    let mut ans = 0;
    for row in 0..map[0].len() {
        for col in 0..map.len() {
            let mut up: usize = 0;
            let mut down: usize = 0;
            let mut left: usize = 0;
            let mut right: usize = 0;

            // Scan up
            for y in (0..row).rev() {
                up = row - y;
                if map[y][col].0 >= map[row][col].0 {
                    break;
                }
            }

            // Scan down
            for y in row + 1..map.len() {
                down = y - row;
                if map[y][col].0 >= map[row][col].0 {
                    break;
                }
            }

            // Scan left
            for x in (0..col).rev() {
                left = col - x;
                if map[row][x].0 >= map[row][col].0 {
                    break;
                }
            }

            // Scan right
            for x in col + 1..map[0].len() {
                right = x - col;
                if map[row][x].0 >= map[row][col].0 {
                    break;
                }
            }

            map[row][col].1 = up * down * left * right;
            if map[row][col].1 > ans {
                ans = map[row][col].1;
            }
        }
    }

    println!("ans: {ans}");
}
