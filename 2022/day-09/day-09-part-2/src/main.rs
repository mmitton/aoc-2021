#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample-2.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(debug_assertions)]
fn dump_map(knots: &[(isize, isize)]) {
    let mut min_x = 0isize;
    let mut max_x = 0isize;
    let mut min_y = 0isize;
    let mut max_y = 0isize;

    for knot in knots {
        if knot.0 < min_x {
            min_x = knot.0;
        }
        if knot.0 > max_x {
            max_x = knot.0;
        }
        if knot.1 < min_y {
            min_y = knot.1;
        }
        if knot.1 > max_y {
            max_y = knot.1;
        }
    }

    println!();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut c = '.';

            if x == 0 && y == 0 {
                c = 's';
            }
            for (idx, knot) in knots.iter().enumerate().rev() {
                if knot.0 == x && knot.1 == y {
                    if idx == 0 {
                        c = 'H';
                    } else {
                        c = (idx as u32 + '0' as u32) as u8 as char;
                    }
                }
            }
            print!("{c}");
        }
        println!();
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut knots = Vec::new();
    for _ in 0..10 {
        knots.push((0isize, 0isize));
    }

    let mut tail_pos = BTreeSet::new();

    for line in &lines {
        if line.is_empty() {
            continue;
        }

        let (dir, cnt) = line.split_once(' ').unwrap();
        let cnt: usize = cnt.parse().unwrap();

        #[cfg(debug_assertions)]
        println!("\n== {dir} {cnt} ==");

        for _ in 0..cnt {
            match dir {
                "U" => knots[0].1 -= 1,
                "D" => knots[0].1 += 1,
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                _ => unreachable!(),
            }

            for i in 1..knots.len() {
                let x_diff = knots[i - 1].0 - knots[i].0;
                let y_diff = knots[i - 1].1 - knots[i].1;

                if y_diff.abs() > 1 {
                    knots[i].1 += if y_diff > 0 { 1 } else { -1 };
                    if x_diff != 0 {
                        knots[i].0 += if x_diff > 0 { 1 } else { -1 };
                    }
                } else if x_diff.abs() > 1 {
                    knots[i].0 += if x_diff > 0 { 1 } else { -1 };
                    if y_diff != 0 {
                        knots[i].1 += if y_diff > 0 { 1 } else { -1 };
                    }
                }
            }

            #[cfg(debug_assertions)]
            dump_map(&knots);

            tail_pos.insert(knots[knots.len() - 1]);
        }
    }

    println!("ans: {}", tail_pos.len());
}
