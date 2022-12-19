const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (isize, isize, isize);

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut grid = BTreeSet::new();
    let mut min: Point = (isize::MAX, isize::MAX, isize::MAX);
    let mut max: Point = (isize::MIN, isize::MIN, isize::MIN);

    for line in lines {
        let nums: Vec<&str> = line.split(',').collect();

        let x: isize = nums[0].parse().unwrap();
        let y: isize = nums[1].parse().unwrap();
        let z: isize = nums[2].parse().unwrap();

        if x <= min.0 {
            min.0 = x - 1;
        }
        if x >= max.0 {
            max.0 = x + 1;
        }
        if y <= min.1 {
            min.1 = y - 1;
        }
        if y >= max.1 {
            max.1 = y + 1;
        }
        if z <= min.2 {
            min.2 = z - 1;
        }
        if z >= max.2 {
            max.2 = z + 1;
        }

        grid.insert((x, y, z));
    }

    let mut ans = 0;
    let mut work = vec![min];
    let mut idx = 0;
    while idx < work.len() {
        let at = work[idx];
        idx += 1;

        macro_rules! process {
            ($x:expr, $y: expr, $z:expr) => {
                let x = $x;
                let y = $y;
                let z = $z;

                if !work.contains(&(x, y, z))
                    && x >= min.0
                    && x <= max.0
                    && y >= min.1
                    && y <= max.1
                    && z >= min.2
                    && z <= max.2
                {
                    if grid.contains(&(x, y, z)) {
                        ans += 1;
                    } else {
                        work.push((x, y, z));
                    }
                }
            };
        }
        process!(at.0 - 1, at.1, at.2);
        process!(at.0 + 1, at.1, at.2);
        process!(at.0, at.1 - 1, at.2);
        process!(at.0, at.1 + 1, at.2);
        process!(at.0, at.1, at.2 - 1);
        process!(at.0, at.1, at.2 + 1);
    }

    println!("ans: {ans}");
    println!("3408 too high");
}
