#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default, Debug)]
struct Sensor {
    id: usize,
    at: [isize; 3],
    beacons: HashSet<[isize; 3]>,
    rotations: Vec<Vec<[isize; 3]>>,
}

impl Sensor {
    const SIN: [isize; 4] = [0, 1, 0, -1];
    const COS: [isize; 4] = [1, 0, -1, 0];
    fn generate_rotations(&mut self) {
        fn rotate_x(points: &mut Vec<[isize; 3]>, rotate_by: usize) {
            for p in points {
                let y = (p[1] * Sensor::COS[rotate_by]) - (p[2] * Sensor::SIN[rotate_by]);
                let z = (p[1] * Sensor::SIN[rotate_by]) + (p[2] * Sensor::COS[rotate_by]);
                p[1] = y;
                p[2] = z;
            }
        }
        fn rotate_y(points: &mut Vec<[isize; 3]>, rotate_by: usize) {
            for p in points {
                let z = (p[2] * Sensor::COS[rotate_by]) - (p[0] * Sensor::SIN[rotate_by]);
                let x = (p[2] * Sensor::SIN[rotate_by]) + (p[0] * Sensor::COS[rotate_by]);
                p[2] = z;
                p[0] = x;
            }
        }
        fn rotate_z(points: &mut Vec<[isize; 3]>, rotate_by: usize) {
            for p in points {
                let x = (p[0] * Sensor::COS[rotate_by]) - (p[1] * Sensor::SIN[rotate_by]);
                let y = (p[0] * Sensor::SIN[rotate_by]) + (p[1] * Sensor::COS[rotate_by]);
                p[0] = x;
                p[1] = y;
            }
        }
        for rotation in 0..24 {
            let axis = rotation / 8;
            let axis_flipped = (rotation % 8) >= 4;
            let axis_rotation = rotation % 4;

            let mut beacons = self.beacons.iter().copied().collect();
            match (axis, axis_flipped) {
                (0, false) => {
                    rotate_x(&mut beacons, axis_rotation);
                    rotate_y(&mut beacons, 0);
                }
                (0, true) => {
                    rotate_x(&mut beacons, axis_rotation);
                    rotate_y(&mut beacons, 2);
                }
                (1, false) => {
                    rotate_y(&mut beacons, axis_rotation);
                    rotate_z(&mut beacons, 1);
                }
                (1, true) => {
                    rotate_y(&mut beacons, axis_rotation);
                    rotate_z(&mut beacons, 3);
                }
                (2, false) => {
                    rotate_z(&mut beacons, axis_rotation);
                    rotate_y(&mut beacons, 1);
                }
                (2, true) => {
                    rotate_z(&mut beacons, axis_rotation);
                    rotate_y(&mut beacons, 3);
                }
                _ => unreachable!(),
            }

            self.rotations.push(beacons);
        }
    }

    fn overlaps(&mut self, other: &Sensor) -> bool {
        let mut moved_points: Vec<[isize; 3]> = Vec::with_capacity(self.beacons.len());
        let other_points: Vec<[isize; 3]> = other.beacons.iter().copied().collect();
        let mut deltas: HashSet<[isize; 3]> = HashSet::default();

        for points in self.rotations.iter() {
            deltas.clear();

            for point in points.iter() {
                for first_point in other_points.iter() {
                    let d0 = first_point[0] - point[0];
                    let d1 = first_point[1] - point[1];
                    let d2 = first_point[2] - point[2];
                    if deltas.insert([d0, d1, d2]) {
                        continue;
                    }

                    moved_points.clear();
                    moved_points.extend(points.iter().map(|p| [p[0] + d0, p[1] + d1, p[2] + d2]));

                    let matches = moved_points
                        .iter()
                        .filter(|p| other.beacons.contains(*p))
                        .count();

                    if matches >= 12 {
                        self.beacons.clear();
                        self.beacons.extend(moved_points.iter());
                        self.at = [d0, d1, d2];
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub struct Day19 {
    sensors: Vec<Sensor>,
}

impl Day19 {
    pub fn new() -> Self {
        Self {
            sensors: Vec::new(),
        }
    }

    fn map(&mut self) {
        for sensor in self.sensors.iter_mut() {
            sensor.generate_rotations();
        }

        let mut remaining_sensors: Vec<Sensor> = self.sensors.drain(..).collect();
        self.sensors.push(remaining_sensors.remove(0));

        let mut i = 0;
        while i < self.sensors.len() {
            let mut j = 0;
            while j < remaining_sensors.len() {
                if remaining_sensors[j].overlaps(&self.sensors[i]) {
                    self.sensors.push(remaining_sensors.remove(j));
                    continue;
                }
                j += 1;
            }
            i += 1;
        }

        assert!(remaining_sensors.is_empty());
    }
}

impl Runner for Day19 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        let mut sensor = Sensor::default();
        for line in lines.iter() {
            if let Some(rest) = line.strip_prefix("--- scanner ") {
                if let Some(rest) = rest.strip_suffix(" ---") {
                    if !sensor.beacons.is_empty() {
                        self.sensors.push(sensor);
                        sensor = Sensor::default();
                    }
                    sensor.id = rest.parse()?;
                } else {
                    return Err(Error::InvalidInput(line.into()));
                }
            } else {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() != 3 {
                    return Err(Error::InvalidInput(line.into()));
                }
                let point: [isize; 3] = [parts[0].parse()?, parts[1].parse()?, parts[2].parse()?];
                sensor.beacons.insert(point);
            }
        }

        if !sensor.beacons.is_empty() {
            self.sensors.push(sensor);
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

impl Day19 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.map();
        let mut beacons: HashSet<[isize; 3]> = HashSet::default();
        for sensor in self.sensors.iter() {
            for beacon in sensor.beacons.iter() {
                beacons.insert(*beacon);
            }
        }
        Ok(beacons.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.map();
        let mut ans = 0;
        for a in self.sensors.iter() {
            for b in self.sensors.iter() {
                let d0 = (a.at[0] - b.at[0]).abs();
                let d1 = (a.at[1] - b.at[1]).abs();
                let d2 = (a.at[2] - b.at[2]).abs();
                ans = ans.max(d0 + d1 + d2);
            }
        }
        Ok(ans.into())
    }
}
