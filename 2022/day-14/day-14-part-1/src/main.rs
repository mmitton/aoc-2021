#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn dump_grid(
    grid: &BTreeMap<(u16, u16), char>,
    min: (u16, u16),
    max: (u16, u16),
    falling: Option<(u16, u16)>,
) {
    let (falling_x, falling_y) = if let Some((x, y)) = falling {
        (x, y)
    } else {
        (u16::MAX, u16::MAX)
    };
    for y in min.1..=max.1 {
        print!("{y:03}  ");
        for x in min.0..=max.0 {
            if let Some(c) = grid.get(&(x, y)) {
                print!("{c}");
            } else if x == falling_x && y == falling_y {
                print!("+");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut grid: BTreeMap<(u16, u16), char> = BTreeMap::new();
    let mut min_x = u16::MAX;
    let mut min_y = u16::MIN;
    let mut max_x = u16::MIN;
    let mut max_y = u16::MIN;
    for line in lines {
        let mut coords: Vec<(u16, u16)> = Vec::new();
        for coord in line.split(" -> ") {
            let (x, y) = coord.split_once(',').unwrap();
            let x: u16 = x.parse().unwrap();
            let y: u16 = y.parse().unwrap();

            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }

            coords.push((x, y));
        }

        for i in 0..coords.len() - 1 {
            let s = &coords[i];
            let e = &coords[i + 1];

            let sx = if s.0 < e.0 { s.0 } else { e.0 };
            let ex = if s.0 > e.0 { s.0 } else { e.0 };
            let sy = if s.1 < e.1 { s.1 } else { e.1 };
            let ey = if s.1 > e.1 { s.1 } else { e.1 };

            for y in sy..=ey {
                for x in sx..=ex {
                    grid.insert((x, y), '#');
                }
            }
        }
    }

    let mut ans = 0;
    'run_loop: loop {
        let mut sand = (500, 0);
        'fall_loop: loop {
            if grid.contains_key(&(sand.0, sand.1 + 1)) {
                // Something is below, scan left
                if grid.contains_key(&(sand.0 - 1, sand.1 + 1)) {
                    // something is to the left
                    if grid.contains_key(&(sand.0 + 1, sand.1 + 1)) {
                        // Somethign is to the right, stay here
                        grid.insert(sand, 'o');
                        break 'fall_loop;
                    } else {
                        sand.0 += 1;
                        sand.1 += 1;
                    }
                } else {
                    sand.0 -= 1;
                    sand.1 += 1;
                }
            } else {
                sand.1 += 1;
                if sand.1 >= max_y {
                    break 'run_loop;
                }
            }
        }
        ans += 1;
    }

    println!("ans: {ans}");
}
