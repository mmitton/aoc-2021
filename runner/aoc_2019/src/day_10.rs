#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::cmp::Ordering;
use std::collections::HashSet;

pub struct Day10 {
    asteroids: HashSet<Point>,
}

impl Day10 {
    pub fn new() -> Self {
        Self {
            asteroids: HashSet::new(),
        }
    }

    fn find_best(&self) -> (Point, usize) {
        let mut best_base = Point::new(0, 0);
        let mut best_seen = 0;
        let mut blocked = HashSet::new();

        for base in self.asteroids.iter() {
            blocked.clear();

            for asteroid in self.asteroids.iter() {
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
                    let factor = helper::gcd(dx.unsigned_abs(), dy.unsigned_abs());
                    dx /= factor as isize;
                    dy /= factor as isize;
                }
                let mut p: Point = Point::new(asteroid.x - dx, asteroid.y - dy);
                while p != *base {
                    if self.asteroids.contains(&p) {
                        blocked.insert(asteroid);
                        break;
                    }
                    p.x -= dx;
                    p.y -= dy;
                }
            }

            let seen = self.asteroids.len() - 1 - blocked.len();
            if seen > best_seen {
                best_seen = seen;
                best_base = *base;
            }
            // println!("  {base:?} can see {seen}");
        }
        (best_base, best_seen)
    }

    fn vaporize(&mut self, base: Point) -> Option<Point> {
        let mut order = Vec::new();
        for asteroid in self.asteroids.iter() {
            if *asteroid == base {
                continue;
            }
            let order_point = LaserPoint::new(asteroid.x - base.x, asteroid.y - base.y);
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
            Some(order[199].1)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Slope {
    rise: usize,
    run: usize,
}

#[allow(clippy::non_canonical_partial_ord_impl)]
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

impl Ord for Slope {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rise.cmp(&other.rise) {
            Ordering::Equal => self.run.cmp(&other.run),
            x => x,
        }
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
            (1, Slope::new(x.unsigned_abs(), y.unsigned_abs()))
        } else if x > 0 && y == 0 {
            (2, Slope::default())
        } else if x > 0 && y > 0 {
            (3, Slope::new(y.unsigned_abs(), x.unsigned_abs()))
        } else if x == 0 && y > 0 {
            (4, Slope::default())
        } else if x < 0 && y > 0 {
            (5, Slope::new(x.unsigned_abs(), y.unsigned_abs()))
        } else if x < 0 && y == 0 {
            (6, Slope::default())
        } else {
            (7, Slope::new(y.unsigned_abs(), x.unsigned_abs()))
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

impl Runner for Day10 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        for (y, line) in Lines::from_path(path, LinesOpt::RAW)?.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let p = Point::new(x as isize, y as isize);
                    self.asteroids.insert(p);
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let (_, best_seen) = self.find_best();
        Ok(best_seen.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let (best_base, _) = self.find_best();
        if let Some(ans) = self.vaporize(best_base) {
            Ok((ans.x * 100 + ans.y).into())
        } else {
            Err(Error::Unsolved)
        }
    }
}
