#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    NoSolution,
}

#[derive(Debug)]
struct Overlap {
    a: usize,
    a_rot: usize,
    a_beacon: usize,
    b: usize,
    b_rot: usize,
    b_beacon: usize,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:2},{:2},{:2}", self.x, self.y, self.z)
    }
}

impl Point {
    fn add(&self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    fn sub(&self, rhs: &Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Point>,
    rotations: Vec<Vec<Point>>,
}

impl Scanner {
    fn generate_rotations(&mut self) {
        fn rotate_x(points: &mut Vec<Point>, rotate_by: usize) {
            let sin: [isize; 4] = [0, 1, 0, -1];
            let cos: [isize; 4] = [1, 0, -1, 0];

            for p in points {
                let y = (p.y * cos[rotate_by]) - (p.z * sin[rotate_by]);
                let z = (p.y * sin[rotate_by]) + (p.z * cos[rotate_by]);
                p.y = y;
                p.z = z;
            }
        }
        fn rotate_y(points: &mut Vec<Point>, rotate_by: usize) {
            let sin: [isize; 4] = [0, 1, 0, -1];
            let cos: [isize; 4] = [1, 0, -1, 0];

            for p in points {
                let z = (p.z * cos[rotate_by]) - (p.x * sin[rotate_by]);
                let x = (p.z * sin[rotate_by]) + (p.x * cos[rotate_by]);
                p.z = z;
                p.x = x;
            }
        }
        fn rotate_z(points: &mut Vec<Point>, rotate_by: usize) {
            let sin: [isize; 4] = [0, 1, 0, -1];
            let cos: [isize; 4] = [1, 0, -1, 0];

            for p in points {
                let x = (p.x * cos[rotate_by]) - (p.y * sin[rotate_by]);
                let y = (p.x * sin[rotate_by]) + (p.y * cos[rotate_by]);
                p.x = x;
                p.y = y;
            }
        }

        for rotation in 0..24 {
            let axis = rotation / 8;
            let axis_flipped = (rotation % 8) >= 4;
            let axis_rotation = rotation % 4;

            let mut beacons = self.beacons.clone();
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
}

fn load_input(filename: &str) -> Result<Vec<Scanner>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut scanners = Vec::new();
    let mut scanner = Scanner {
        beacons: Vec::new(),
        rotations: Vec::new(),
    };

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if line.starts_with("#") {
            continue;
        }

        if line.starts_with("---") {
            if scanner.beacons.len() > 0 {
                scanner.generate_rotations();
                scanners.push(scanner);
                scanner = Scanner {
                    beacons: Vec::new(),
                    rotations: Vec::new(),
                };
            }
        } else {
            let parts: Vec<&str> = line.split(",").collect();
            scanner.beacons.push(Point {
                x: parts[0].parse().map_err(|e| Error::NAN(e))?,
                y: parts[1].parse().map_err(|e| Error::NAN(e))?,
                z: parts[2].parse().map_err(|e| Error::NAN(e))?,
            });
        }
    }

    if scanner.beacons.len() > 0 {
        scanner.generate_rotations();
        scanners.push(scanner);
    }
    Ok(scanners)
}

fn find_beacons(scanners: &Vec<Scanner>, overlaps: &Vec<Vec<Overlap>>) -> Option<Vec<Point>> {
    fn iter(
        scanners: &Vec<Scanner>,
        overlaps: &Vec<Vec<Overlap>>,
        beacons: Vec<Point>,
        positions: Vec<Option<(usize, Point)>>,
        found: usize,
    ) -> Option<Vec<Point>> {
        println!("Positions: {:?}", positions);
        if found == scanners.len() {
            let mut beacons = beacons.clone();
            beacons.sort();
            return Some(beacons);
        }

        for a in 0..scanners.len() {
            if let Some((rot, p)) = positions[a] {
                for b in 0..scanners.len() {
                    if positions[b].is_some() {
                        continue;
                    }

                    for k in 0..overlaps[a].len() {
                        assert!(overlaps[a][k].a == a);
                        if overlaps[a][k].a_rot != rot {
                            continue;
                        }
                        if overlaps[a][k].b != b {
                            continue;
                        }

                        let a_rot = overlaps[a][k].a_rot;
                        let a_beacon = overlaps[a][k].a_beacon;
                        let b_rot = overlaps[a][k].b_rot;
                        let b_beacon = overlaps[a][k].b_beacon;
                        println!("Let do {} {} => {} {}", a, a_rot, b, b_rot);

                        let translate = scanners[a].rotations[a_rot][a_beacon]
                            .sub(&scanners[b].rotations[b_rot][b_beacon])
                            .add(&p);

                        // Ok, let's run down this path
                        let mut new_beacons = beacons.clone();
                        for p in &scanners[b].rotations[b_rot] {
                            let p = p.add(&translate);
                            if !new_beacons.contains(&p) {
                                new_beacons.push(p);
                            }
                        }

                        let mut new_positions = positions.clone();
                        new_positions[b] = Some((b_rot, translate));

                        if let Some(beacons) =
                            iter(scanners, overlaps, new_beacons, new_positions, found + 1)
                        {
                            return Some(beacons);
                        }
                    }
                }
            }
        }

        None
    }

    let init_beacons = scanners[0].rotations[0].clone();
    let mut init_positions: Vec<Option<(usize, Point)>> = vec![None; scanners.len()];
    init_positions[0] = Some((0, Point { x: 0, y: 0, z: 0 }));

    iter(scanners, overlaps, init_beacons, init_positions, 1)
}

fn main() -> Result<(), Error> {
    let scanners = load_input(INPUT_FILE)?;

    let mut look_at = Vec::new();
    look_at.push((0, 0));

    let mut overlaps: Vec<Vec<Overlap>> = Vec::new();
    for _ in 0..scanners.len() {
        overlaps.push(Vec::new());
    }

    let mut p = Point::default();
    println!("Finding overlaps");

    let mut i = 0;
    while i != look_at.len() {
        let a = look_at[i].0;
        let a_rot = look_at[i].1;

        println!("a:{}  a_rot:{}", a, a_rot);

        for b in 1..scanners.len() {
            for b_rot in 0..24 {
                if look_at.contains(&(b, b_rot)) {
                    continue;
                }

                // Loop through the rotated beacons in a
                'outside_loop: for b1 in 0..scanners[a].beacons.len() {
                    // Loop through the rotated beacons in b
                    for b2 in 0..scanners[b].beacons.len() {
                        // Find a new translation based on a and b
                        let pos =
                            scanners[b].rotations[b_rot][b2].sub(&scanners[a].rotations[a_rot][b1]);

                        // Count how many beacons match between the two, should be at least one or
                        // else "I'M DUMB"
                        let mut match_count = 0;
                        for b3 in 0..scanners[a].beacons.len() {
                            p.x = scanners[a].rotations[a_rot][b3].x + pos.x;
                            p.y = scanners[a].rotations[a_rot][b3].y + pos.y;
                            p.z = scanners[a].rotations[a_rot][b3].z + pos.z;
                            if scanners[b].rotations[b_rot].contains(&p) {
                                match_count += 1;
                            }
                        }

                        assert!(match_count >= 1, "I'M DUMB");

                        if match_count >= 12 {
                            println!(
                                    "Overlap a:{} a_rot:{} a_beacon:{}  b:{} b_rot:{} b_beacon:{}  match_count:{}",
                                    a, a_rot, b1, b, b_rot, b2, match_count
                                );
                            overlaps[a].push(Overlap {
                                a: a,
                                a_rot: a_rot,
                                a_beacon: b1,
                                b: b,
                                b_rot: b_rot,
                                b_beacon: b2,
                            });
                            overlaps[b].push(Overlap {
                                a: b,
                                a_rot: b_rot,
                                a_beacon: b2,
                                b: a,
                                b_rot: a_rot,
                                b_beacon: b1,
                            });
                            look_at.push((b, b_rot));

                            break 'outside_loop;
                        }
                    }
                }
            }
        }

        i += 1;
    }

    println!("Finished finding overlaps");

    let beacons = find_beacons(&scanners, &overlaps);
    if let Some(beacons) = beacons {
        for beacon in &beacons {
            println!("  {:?}", beacon);
        }
        println!("Number of beacons: {}", beacons.len());

        Ok(())
    } else {
        Err(Error::NoSolution)
    }
}
