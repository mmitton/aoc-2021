const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample-2.txt"
} else {
    "../input.txt"
};

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialEq, Eq)]
struct Slope {
    rise: usize,
    run: usize,
}

impl PartialOrd for Slope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rise == other.rise && self.run == other.run {
            return Some(Ordering::Equal);
        }

        if self.rise == 0 && self.run == 0 {
            return None;
        }
        if other.rise == 0 && other.run == 0 {
            return None;
        }

        let s1 = self.rise as f64 / self.run as f64;
        let s2 = other.rise as f64 / other.run as f64;

        s1.partial_cmp(&s2)
    }
}

impl Slope {
    fn new(mut rise: usize, mut run: usize) -> Self {
        for factor in (2..=rise).rev() {
            if rise % factor == 0 && run % factor == 0 {
                rise /= factor;
                run /= factor;
                break;
            }
        }

        Self { rise, run }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct LaserPoint {
    quad: u8,
    slope: Slope,
    sq_dist: usize,
    x: isize,
    y: isize,
    destroy: Option<usize>,
}

impl LaserPoint {
    fn new(x: isize, y: isize) -> Self {
        let sq_dist = (x * x) + (y * y);
        let (quad, slope) = if x == 0 && y <= 0 {
            (0, Slope::default())
        } else if x > 0 && y < 0 {
            (1, Slope::new(x.abs() as usize, y.abs() as usize))
        } else if x > 0 && y == 0 {
            (2, Slope::default())
        } else if x > 0 && y > 0 {
            (3, Slope::new(y.abs() as usize, x.abs() as usize))
        } else if x == 0 && y > 0 {
            (4, Slope::default())
        } else if x < 0 && y > 0 {
            (5, Slope::new(x.abs() as usize, y.abs() as usize))
        } else if x < 0 && y == 0 {
            (6, Slope::default())
        } else {
            (7, Slope::new(y.abs() as usize, x.abs() as usize))
        };
        Self {
            sq_dist: sq_dist as usize,
            quad,
            slope,
            x,
            y,
            destroy: None,
        }
    }
}

// impl PartialOrd for Point {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//
//     }
// }

fn find_best(asteroids: &mut BTreeMap<Point, bool>, start_at: Option<Point>) {
    if asteroids.is_empty() {
        return;
    }

    let bases: Vec<Point> = asteroids.keys().cloned().collect();
    let best_base = if let Some(start_at) = start_at {
        start_at
    } else {
        let mut best_base = bases[0];
        let mut best_seen = 0;
        for base in &bases {
            let mut max_x = isize::MIN;
            let mut max_y = isize::MIN;
            for (asteroid, blocked) in asteroids.iter_mut() {
                if asteroid.x > max_x {
                    max_x = asteroid.x;
                }
                if asteroid.y > max_y {
                    max_y = asteroid.y;
                }
                *blocked = false;
            }
            // println!("\n{base:?}");

            for asteroid in &bases {
                if asteroid == base {
                    continue;
                }
                let mut dx = asteroid.x - base.x;
                let mut dy = asteroid.y - base.y;
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
                let mut p: Point = Point::new(asteroid.x - dx, asteroid.y - dy);
                while p != *base {
                    if asteroids.contains_key(&p) {
                        // println!("{asteroid:?} is blocked by {p:?}");
                        *asteroids.get_mut(asteroid).unwrap() = true;
                        break;
                    }
                    p.x -= dx;
                    p.y -= dy;
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
        best_base
    };

    let mut order = Vec::new();
    for asteroid in &bases {
        if *asteroid == best_base {
            continue;
        }
        let order_point = LaserPoint::new(asteroid.x - best_base.x, asteroid.y - best_base.y);
        order.push((order_point, *asteroid));
    }
    order.sort();

    // TODO: Destroy stuff
    let mut num_destoyed = 0;
    while num_destoyed != order.len() {
        let mut last_quad = 8;
        let mut last_slope = Slope::default();
        for p in order.iter_mut() {
            if p.0.destroy.is_some() {
                continue;
            }
            if p.0.quad == last_quad && p.0.slope == last_slope {
                continue;
            }

            num_destoyed += 1;
            p.0.destroy = Some(num_destoyed);
            last_quad = p.0.quad;
            last_slope = p.0.slope;
        }
    }

    order.sort_by_key(|p| p.0.destroy);

    if order.len() >= 200 {
        let base = order[199];
        println!("{} : {},{}", base.0.destroy.unwrap(), base.1.x, base.1.y);
        println!("ans: {}", base.1.x * 100 + base.1.y);
    } else {
        for base in order.iter() {
            println!("{} : {},{}", base.0.destroy.unwrap(), base.1.x, base.1.y);
        }
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut asteroids: BTreeMap<Point, bool> = BTreeMap::new();
    let mut y: isize = -1;
    let mut start_at: Option<Point> = None;
    for line in lines.iter() {
        if line.is_empty() {
            find_best(&mut asteroids, start_at);
            start_at = None;
            asteroids.clear();
            y = -1;
            continue;
        }

        y += 1;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let p = Point::new(x as isize, y);
                if c == 'X' {
                    start_at = Some(p);
                }
                asteroids.insert(p, false);
            }
        }
    }

    find_best(&mut asteroids, start_at);
}
