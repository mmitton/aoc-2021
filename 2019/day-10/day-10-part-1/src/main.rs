const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Point(isize, isize);

fn find_best(asteroids: &mut BTreeMap<Point, bool>) {
    if asteroids.is_empty() {
        return;
    }

    let bases: Vec<Point> = asteroids.keys().cloned().collect();
    let mut best_base = bases[0];
    let mut best_seen = 0;
    for base in &bases {
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;
        for (asteroid, blocked) in asteroids.iter_mut() {
            if asteroid.0 > max_x {
                max_x = asteroid.0;
            }
            if asteroid.1 > max_y {
                max_y = asteroid.1;
            }
            *blocked = false;
        }
        // println!("\n{base:?}");

        for asteroid in &bases {
            if asteroid == base {
                continue;
            }
            let mut dx = asteroid.0 - base.0;
            let mut dy = asteroid.1 - base.1;
            // print!("  dx:{dx} dy:{dy} => ");
            if dx == 0 {
                dy = if dy < 0 { -1 } else { 1 };
            } else if dy == 0 {
                dx = if dx < 0 { -1 } else { 1 };
            } else {
                for factor in (2..=dx.abs()).rev() {
                    if dx % factor == 0 && dy % factor == 0 {
                        dx /= factor;
                        dy /= factor;
                        break;
                    }
                }
            }
            // println!("  {base:?}  asteroid:{asteroid:?}  dx:{dx} dy:{dy}");
            let mut p = Point(asteroid.0 - dx, asteroid.1 - dy);
            while p != *base {
                if asteroids.contains_key(&p) {
                    // println!("{asteroid:?} is blocked by {p:?}");
                    *asteroids.get_mut(asteroid).unwrap() = true;
                    break;
                }
                p.0 -= dx;
                p.1 -= dy;
            }
        }

        let mut seen = 0;
        for (asteroid, blocked) in asteroids.iter() {
            if asteroid == base {
                continue;
            }
            if !*blocked {
                seen += 1;
            }
        }
        if seen > best_seen {
            best_seen = seen;
            best_base = *base;
        }
        // println!("  {base:?} can see {seen}");
    }

    println!("Best base is {best_base:?} with {best_seen} seen");
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut asteroids: BTreeMap<Point, bool> = BTreeMap::new();
    let mut y: isize = -1;
    for line in lines.iter() {
        if line.is_empty() {
            find_best(&mut asteroids);
            asteroids.clear();
            y = -1;
            continue;
        }

        y += 1;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                asteroids.insert(Point(x as isize, y), false);
            }
        }
    }

    find_best(&mut asteroids);
}
