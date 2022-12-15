const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    d: isize,

    check_points: Vec<(isize, isize)>,
}

impl Sensor {
    fn new(x: isize, y: isize, d: isize) -> Self {
        Self {
            x,
            y,
            d,
            check_points: vec![
                (x - d - 1, y),
                (x, y - d - 1),
                (x + d + 1, y),
                (x, y + d + 1),
            ],
        }
    }

    fn check_points(&self, min: isize, max: isize) -> Vec<(isize, isize)> {
        let mut check_points = Vec::new();

        macro_rules! add {
            ($from:expr, $to:expr) => {{
                let mut p = $from;
                while p != $to {
                    if p.0 >= min && p.0 <= max && p.1 >= min && p.1 <= max {
                        check_points.push(p);
                    }
                    p.0 += if p.0 < $to.0 { 1 } else { -1 };
                    p.1 += if p.1 < $to.1 { 1 } else { -1 };
                }
            }};
        }

        add!(self.check_points[0], self.check_points[1]);
        add!(self.check_points[1], self.check_points[2]);
        add!(self.check_points[2], self.check_points[3]);
        add!(self.check_points[3], self.check_points[0]);

        check_points
    }

    fn can_see(&self, x: isize, y: isize) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.d
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    const SIZE: isize = if cfg!(debug_assertions) { 20 } else { 4000000 };

    let mut sensors = Vec::new();

    for line in lines {
        let line = line.replace('=', " ").replace(',', "").replace(':', "");
        let parts: Vec<&str> = line.split(' ').collect();
        let sx: isize = parts[3].parse().unwrap();
        let sy: isize = parts[5].parse().unwrap();
        let bx: isize = parts[11].parse().unwrap();
        let by: isize = parts[13].parse().unwrap();

        let d = (sx - bx).abs() + (sy - by).abs();

        sensors.push(Sensor::new(sx, sy, d));
    }

    for i in 0..sensors.len() {
        let check_points = sensors[i].check_points(0, SIZE);
        'check_points: for cp in &check_points {
            for j in 0..sensors.len() {
                if i == j {
                    continue;
                }
                if sensors[j].can_see(cp.0, cp.1) {
                    continue 'check_points;
                }
            }
            let x = cp.0;
            let y = cp.1;
            let ans = (x * 4000000) + y;
            println!("{x},{y}  :  {ans}");
            return;
        }
    }
}
