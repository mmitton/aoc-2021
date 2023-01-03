const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };
    lines.retain(|l| !l.is_empty());

    for lines in lines.chunks(2) {
        #[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
        struct Point(isize, isize);

        let mut grid: BTreeMap<Point, Vec<Option<usize>>> = BTreeMap::new();

        for (idx, line) in lines.iter().enumerate() {
            let mut p = Point(0, 0);
            let mut dist = 0;
            for m in line.split(',') {
                let dir = &m[0..1];
                let cnt: isize = m[1..].parse().unwrap();
                let delta = match dir {
                    "U" => Point(0, -1),
                    "D" => Point(0, 1),
                    "L" => Point(-1, 0),
                    "R" => Point(1, 0),
                    _ => unreachable!(),
                };
                for _ in 0..cnt {
                    dist += 1;
                    p.0 += delta.0;
                    p.1 += delta.1;
                    let grid = grid.entry(p).or_insert(vec![None; 2]);
                    grid[idx] = Some(dist);
                }
            }
        }
        grid.retain(|_k, v| v[0].is_some() && v[1].is_some());
        let mut ans = usize::MAX;
        for v in grid.values() {
            let dist = v[0].unwrap() + v[1].unwrap();
            if dist < ans {
                ans = dist;
            }
        }
        println!("ans: {ans}");
    }
}
