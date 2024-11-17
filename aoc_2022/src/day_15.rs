#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Sensor {
    x: isize,
    y: isize,
    d: isize,
    bx: isize,
    by: isize,

    check_points: [(isize, isize, isize, isize); 4],
}

impl Sensor {
    fn new(x: isize, y: isize, bx: isize, by: isize) -> Self {
        let d = (x - bx).abs() + (y - by).abs();

        Self {
            x,
            y,
            d,
            bx,
            by,
            check_points: [
                (x - d - 1, y, 1, -1),
                (x, y - d - 1, 1, 1),
                (x + d + 1, y, -1, 1),
                (x, y + d + 1, -1, -1),
            ],
        }
    }

    // fn check_points(&self, min: isize, max: isize) -> Vec<(isize, isize)> {
    //     let mut check_points = Vec::new();

    //     macro_rules! add {
    //         ($from:expr, $to:expr) => {{
    //             let mut p = $from;
    //             while p != $to {
    //                 if p.0 >= min && p.0 <= max && p.1 >= min && p.1 <= max {
    //                     check_points.push(p);
    //                 }
    //                 p.0 += if p.0 < $to.0 { 1 } else { -1 };
    //                 p.1 += if p.1 < $to.1 { 1 } else { -1 };
    //             }
    //         }};
    //     }

    //     add!(self.check_points[0], self.check_points[1]);
    //     add!(self.check_points[1], self.check_points[2]);
    //     add!(self.check_points[2], self.check_points[3]);
    //     add!(self.check_points[3], self.check_points[0]);

    //     check_points
    // }

    fn can_see(&self, x: isize, y: isize) -> bool {
        let dx = (self.x - x).abs();
        let dy = (self.y - y).abs();
        dx <= self.d && dy <= self.d && dx + dy <= self.d
    }
}

pub struct Day15 {
    sensors: Vec<Sensor>,
}

impl Day15 {
    pub fn new() -> Self {
        Self {
            sensors: Vec::new(),
        }
    }
}

impl Runner for Day15 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let line = line.replace('=', " ").replace([',', ':'], "");
            let parts: Vec<&str> = line.split(' ').collect();
            let sx: isize = parts[3].parse().unwrap();
            let sy: isize = parts[5].parse().unwrap();
            let bx: isize = parts[11].parse().unwrap();
            let by: isize = parts[13].parse().unwrap();

            self.sensors.push(Sensor::new(sx, sy, bx, by));
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day15 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let row = if self.sensors[0].y < 100 { 10 } else { 2000000 };
        let mut seen: Vec<(isize, isize)> = Vec::new();

        let mut beacons = Vec::new();
        for sensor in self.sensors.iter() {
            if sensor.by == row {
                beacons.push(sensor.bx);
            }
            if (sensor.y - row).abs() > sensor.d {
                continue;
            }
            let dx = sensor.d - (sensor.y - row).abs();
            seen.push((sensor.x - dx, sensor.x + dx));
        }
        seen.sort();
        beacons.sort();
        beacons.dedup();
        let mut last = isize::MIN;
        let mut ans = -(beacons.len() as isize);
        for &(from, to) in seen.iter() {
            if to <= last {
                continue;
            }
            let from = if from <= last { last + 1 } else { from };
            let to_add = to - from + 1;
            ans += to_add;
            last = to;
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let size: isize = if self.sensors[0].y < 100 { 20 } else { 4000000 };
        let range = 0..=size;
        for i in 0..self.sensors.len() {
            let mut sensor = self.sensors[i];
            let near_sensors: Vec<&Sensor> = self
                .sensors
                .iter()
                .filter(|s| {
                    if *s == &sensor {
                        false
                    } else {
                        (sensor.x - s.x).abs() + (sensor.y - s.y).abs() <= sensor.d + s.d + 1
                    }
                })
                .collect();
            for _ in 0..=self.sensors[i].d {
                'check_points: for cp in sensor.check_points.iter() {
                    if range.contains(&cp.0) && range.contains(&cp.1) {
                        for s in near_sensors.iter() {
                            if s.can_see(cp.0, cp.1) {
                                continue 'check_points;
                            }
                        }
                        let x = cp.0;
                        let y = cp.1;
                        let ans = (x * 4000000) + y;
                        return Ok(ans.into());
                    }
                }
                for cp in sensor.check_points.iter_mut() {
                    cp.0 += cp.2;
                    cp.1 += cp.3;
                }
            }
        }

        Err(Error::Unsolved)
    }
}
