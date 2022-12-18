const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut grid = BTreeSet::new();
    for line in lines {
        let nums: Vec<&str> = line.split(',').collect();

        let x: isize = nums[0].parse().unwrap();
        let y: isize = nums[1].parse().unwrap();
        let z: isize = nums[2].parse().unwrap();

        grid.insert((x, y, z));
    }

    let mut ans = 0;
    for (x, y, z) in grid.iter() {
        let x = *x;
        let y = *y;
        let z = *z;

        if !grid.contains(&(x + 1, y, z)) {
            ans += 1;
        }
        if !grid.contains(&(x - 1, y, z)) {
            ans += 1;
        }
        if !grid.contains(&(x, y + 1, z)) {
            ans += 1;
        }
        if !grid.contains(&(x, y - 1, z)) {
            ans += 1;
        }
        if !grid.contains(&(x, y, z + 1)) {
            ans += 1;
        }
        if !grid.contains(&(x, y, z - 1)) {
            ans += 1;
        }
    }

    println!("ans: {ans}");
}
