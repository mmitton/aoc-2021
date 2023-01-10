const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}

type Bugs = BTreeSet<Coord>;

fn print_bugs(iter: usize, bugs: &Bugs) {
    println!("After {iter} minute(s)");
    for y in 0..5 {
        for x in 0..5 {
            print!(
                "{}",
                if bugs.contains(&Coord { x, y }) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
    println!();
}

fn process(bugs: &Bugs) {
    if bugs.is_empty() {
        return;
    }

    fn get_neighbors(bugs: &Bugs, c: &Coord) -> usize {
        let mut num_neighbors = 0;
        let neighbors = vec![
            Coord { x: c.x - 1, y: c.y },
            Coord { x: c.x + 1, y: c.y },
            Coord { x: c.x, y: c.y - 1 },
            Coord { x: c.x, y: c.y + 1 },
        ];
        for neighbor in &neighbors {
            if bugs.contains(neighbor) {
                num_neighbors += 1;
            }
        }

        num_neighbors
    }

    let mut bugs = bugs.clone();
    print_bugs(0, &bugs);
    let mut history: Vec<Bugs> = Vec::new();
    for iter in 1..=usize::MAX {
        let mut next_bugs = BTreeSet::new();
        for y in 0..5 {
            for x in 0..5 {
                let pos = Coord { x, y };
                let num_neighbors = get_neighbors(&bugs, &pos);
                let is_alive = bugs.contains(&pos);
                if num_neighbors == 1 || (!is_alive && num_neighbors == 2) {
                    next_bugs.insert(pos);
                }
            }
        }
        history.push(bugs);
        bugs = next_bugs;
        print_bugs(iter, &bugs);
        if history.contains(&bugs) {
            let mut ans = 0;
            for c in bugs {
                let idx = (c.y * 5) + c.x;
                ans += 1 << idx;
            }
            println!("ans: {ans}");
            return;
        }
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut bugs: Bugs = Bugs::new();
    let mut y = 0;
    for line in lines.iter() {
        if line.is_empty() {
            process(&bugs);
            bugs.clear();
            y = 0;
            continue;
        }

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                bugs.insert(Coord { x: x as isize, y });
            }
        }
        y += 1;
    }
    process(&bugs);
}
