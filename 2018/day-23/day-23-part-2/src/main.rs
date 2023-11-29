#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample-2.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn scale(&self, shift: u32) -> Self {
        Self {
            x: self.x >> shift,
            y: self.y >> shift,
            z: self.z >> shift,
        }
    }

    fn dist(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Copy, Clone, Debug)]
struct NanoBot {
    center: Point,
    radius: isize,
}

impl NanoBot {
    fn limits(&self) -> [Point; 6] {
        [
            Point::new(self.center.x - self.radius, self.center.y, self.center.z),
            Point::new(self.center.x + self.radius, self.center.y, self.center.z),
            Point::new(self.center.x, self.center.y - self.radius, self.center.z),
            Point::new(self.center.x, self.center.y + self.radius, self.center.z),
            Point::new(self.center.x, self.center.y, self.center.z - self.radius),
            Point::new(self.center.x, self.center.y, self.center.z + self.radius),
        ]
    }

    fn scale(&self, shift: u32) -> Self {
        let center = self.center.scale(shift);
        let mut radius = 0;
        for limit in self.limits().map(|p| p.scale(shift)) {
            radius = radius.max(center.dist(&limit));
        }
        Self { center, radius }
    }

    fn can_see(&self, p: &Point) -> bool {
        self.center.dist(p) <= self.radius
    }
}

fn load_input(filename: &str) -> Result<Vec<NanoBot>, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename)?;

    let lines = BufReader::new(f).lines();
    let mut nanobots = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim().to_string();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split(">, r=").collect();
        let r = parts[1].parse().expect("Could not parse int");
        let parts: Vec<&str> = parts[0][5..].split(',').collect();
        let x = parts[0].parse().expect("Could not parse int");
        let y = parts[1].parse().expect("Could not parse int");
        let z = parts[2].parse().expect("Could not parse int");

        nanobots.push(NanoBot {
            center: Point { x, y, z },
            radius: r,
        });
    }

    Ok(nanobots)
}

fn main() {
    let nanobots = load_input(INPUT_FILE).expect("Could not read input");

    let mut search_spaces: Vec<Point> = Vec::new();
    let mut next_search_spaces: Vec<Point> = Vec::new();
    let mut scaled_nanobots = Vec::with_capacity(nanobots.len());
    let mut seen: BTreeSet<Point> = BTreeSet::new();

    search_spaces.push(Point::new(0, 0, 0));
    for shift in (0..isize::BITS).rev() {
        scaled_nanobots.clear();
        scaled_nanobots.extend(nanobots.iter().map(|nanobot| nanobot.scale(shift)));

        let mut max_seen = 0;
        next_search_spaces.clear();
        seen.clear();
        for point in search_spaces.iter() {
            for z in (point.z * 2) - 1..=(point.z * 2) + 1 {
                for y in (point.y * 2) - 1..=(point.y * 2) + 1 {
                    for x in (point.x * 2) - 1..=(point.x * 2) + 1 {
                        let point = Point::new(x, y, z);
                        if seen.insert(point) {
                            let mut seen = 0;
                            for nanobot in scaled_nanobots.iter() {
                                if nanobot.can_see(&point) {
                                    seen += 1;
                                }
                            }

                            if seen > max_seen {
                                next_search_spaces.clear();
                                max_seen = seen;
                            }
                            if seen == max_seen {
                                next_search_spaces.push(point);
                            }
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut search_spaces, &mut next_search_spaces);
    }

    let origin = Point::new(0, 0, 0);
    let mut closest = isize::MAX;

    for point in search_spaces {
        let dist = origin.dist(&point);
        if dist < closest {
            closest = dist;
        }
    }

    println!("Closest: {closest:?}");
}
