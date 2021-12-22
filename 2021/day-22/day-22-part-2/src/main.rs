#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x: x, y: y, z: z }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Cuboid {
    start: Point,
    end: Point,
}

impl std::fmt::Display for Cuboid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "({},{},{}) -> ({},{},{})",
            self.start.x, self.start.y, self.start.z, self.end.x, self.end.y, self.end.z,
        )
    }
}

impl Cuboid {
    fn new(start: Point, end: Point) -> Self {
        assert!(start.x <= end.x);
        assert!(start.y <= end.y);
        assert!(start.z <= end.z);
        Self {
            start: start,
            end: end,
        }
    }

    /*
    fn fully_overlaps(&self, rhs: &Self) -> bool {
        rhs.start.x >= self.start.x
            && rhs.start.y >= self.start.y
            && rhs.start.z >= self.start.z
            && rhs.end.x <= self.end.x
            && rhs.end.y <= self.end.y
            && rhs.end.z <= self.end.z
    }
    */

    // Returns `None` if no overlap, else returns `Some`(`Vec`<`Cuboid`>) of sections which do not
    // overlap (these will get added in to the reactor, while this `Cuboid` will be removed)
    fn overlaps(&self, rhs: &Self) -> Option<Vec<Cuboid>> {
        // Case 1, self is fully inside rhs
        if self.start.x >= rhs.start.x
            && self.start.y >= rhs.start.y
            && self.start.z >= rhs.start.z
            && self.end.x <= rhs.end.x
            && self.end.y <= rhs.end.y
            && self.end.z <= rhs.end.z
        {
            return Some(Vec::new());
        }

        // Check if rhs is fully outside self
        if rhs.end.x < self.start.x
            || rhs.start.x > self.end.x
            || rhs.end.y < self.start.y
            || rhs.start.y > self.end.y
            || rhs.end.z < self.start.z
            || rhs.start.z > self.end.z
        {
            return None;
        }

        // Now shrink rhs to fit within self
        let rhs_start_x = if rhs.start.x < self.start.x {
            self.start.x
        } else {
            rhs.start.x
        };
        let rhs_end_x = if rhs.end.x > self.end.x {
            self.end.x
        } else {
            rhs.end.x
        };
        let rhs_start_y = if rhs.start.y < self.start.y {
            self.start.y
        } else {
            rhs.start.y
        };
        let rhs_end_y = if rhs.end.y > self.end.y {
            self.end.y
        } else {
            rhs.end.y
        };
        let rhs_start_z = if rhs.start.z < self.start.z {
            self.start.z
        } else {
            rhs.start.z
        };
        let rhs_end_z = if rhs.end.z > self.end.z {
            self.end.z
        } else {
            rhs.end.z
        };

        /*
        println!("self: {:?}", self);
        println!("rhs: {:?}", rhs);
        println!("rhs_start_x:{}  rhs_end_x:{}", rhs_start_x, rhs_end_x);
        println!("rhs_start_y:{}  rhs_end_y:{}", rhs_start_y, rhs_end_y);
        println!("rhs_start_z:{}  rhs_end_z:{}", rhs_start_z, rhs_end_z);
        */

        let mut non_overlaps = Vec::new();

        // Non Overlapping section behind rhs in the Z axis
        if rhs_start_z != self.start.z {
            non_overlaps.push(Cuboid::new(
                Point::new(self.start.x, self.start.y, self.start.z),
                Point::new(self.end.x, self.end.y, rhs_start_z - 1),
            ));
        }
        // Non Overlapping section in front rhs in the Z axis
        if rhs_end_z != self.end.z {
            non_overlaps.push(Cuboid::new(
                Point::new(self.start.x, self.start.y, rhs_end_z + 1),
                Point::new(self.end.x, self.end.y, self.end.z),
            ));
        }

        // Non Overlapping sections in the same z plane as rhs
        if rhs_start_y != self.start.y {
            if rhs_start_x != self.start.x {
                non_overlaps.push(Cuboid::new(
                    Point::new(self.start.x, self.start.y, rhs_start_z),
                    Point::new(rhs_start_x - 1, rhs_start_y - 1, rhs_end_z),
                ));
            }
            non_overlaps.push(Cuboid::new(
                Point::new(rhs_start_x, self.start.y, rhs_start_z),
                Point::new(rhs_end_x, rhs_start_y - 1, rhs_end_z),
            ));
            if rhs_end_x != self.end.x {
                non_overlaps.push(Cuboid::new(
                    Point::new(rhs_end_x + 1, self.start.y, rhs_start_z),
                    Point::new(self.end.x, rhs_start_y - 1, rhs_end_z),
                ));
            }
        }

        if rhs_start_x != self.start.x {
            non_overlaps.push(Cuboid::new(
                Point::new(self.start.x, rhs_start_y, rhs_start_z),
                Point::new(rhs_start_x - 1, rhs_end_y, rhs_end_z),
            ));
        }
        if rhs_end_x != self.end.x {
            non_overlaps.push(Cuboid::new(
                Point::new(rhs_end_x + 1, rhs_start_y, rhs_start_z),
                Point::new(self.end.x, rhs_end_y, rhs_end_z),
            ));
        }

        if rhs_end_y != self.end.y {
            if rhs_start_x != self.start.x {
                non_overlaps.push(Cuboid::new(
                    Point::new(self.start.x, rhs_end_y + 1, rhs_start_z),
                    Point::new(rhs_start_x - 1, self.end.y, rhs_end_z),
                ));
            }
            non_overlaps.push(Cuboid::new(
                Point::new(rhs_start_x, rhs_end_y + 1, rhs_start_z),
                Point::new(rhs_end_x, self.end.y, rhs_end_z),
            ));
            if rhs_end_x != self.end.x {
                non_overlaps.push(Cuboid::new(
                    Point::new(rhs_end_x + 1, rhs_end_y + 1, rhs_start_z),
                    Point::new(self.end.x, self.end.y, rhs_end_z),
                ));
            }
        }
        return Some(non_overlaps);
    }
}

#[derive(Debug)]
struct Reactor {
    cuboids: Vec<Cuboid>,
}

impl Reactor {
    fn new() -> Self {
        Self {
            cuboids: Vec::new(),
        }
    }

    fn merge_step(&mut self, turn_on: bool, cuboid: &Cuboid) {
        // For both turn on and turn off, remove cuboids which overlap and add back in parts which
        // do not as new cuboids
        let mut to_add = Vec::new();
        self.cuboids.retain(|c| {
            if let Some(non_overlaps) = c.overlaps(cuboid) {
                to_add.extend_from_slice(&non_overlaps);
                false
            } else {
                true
            }
        });
        self.cuboids.extend_from_slice(&to_add);

        // Finally, if this is a turn on, add it to the list
        if turn_on {
            // Turn On
            self.cuboids.push(cuboid.clone());
        }

        if cfg!(debug_assertions) {
            // Now sort the cuboids, just for fun
            // self.cuboids.sort();
        }
    }

    fn full_count(&self) -> usize {
        let mut on = 0usize;
        for c in &self.cuboids {
            on += (c.end.x - c.start.x + 1) as usize
                * (c.end.y - c.start.y + 1) as usize
                * (c.end.z - c.start.z + 1) as usize;
        }

        on
    }

    fn count(&self, area: &Cuboid) -> usize {
        let mut on = 0usize;
        for z in area.start.z..=area.end.z {
            for y in area.start.y..=area.end.y {
                for x in area.start.x..=area.end.x {
                    for c in &self.cuboids {
                        if c.start.x <= x
                            && c.end.x >= x
                            && c.start.y <= y
                            && c.end.y >= y
                            && c.start.z <= z
                            && c.end.z >= z
                        {
                            on += 1;
                            break;
                        }
                    }
                }
            }
        }

        on
    }
}

fn load_input(filename: &str) -> Result<Vec<(bool, Cuboid)>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut steps: Vec<(bool, Cuboid)> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        let coords = parts[1].replace("x=", "");
        let coords = coords.replace(",y=", " ");
        let coords = coords.replace(",z=", " ");
        let coords = coords.replace("..", " ");
        let coords: Vec<&str> = coords.split(" ").collect();

        let cuboid = Cuboid {
            start: Point {
                x: coords[0].parse().map_err(|e| Error::NAN(e))?,
                y: coords[2].parse().map_err(|e| Error::NAN(e))?,
                z: coords[4].parse().map_err(|e| Error::NAN(e))?,
            },
            end: Point {
                x: coords[1].parse().map_err(|e| Error::NAN(e))?,
                y: coords[3].parse().map_err(|e| Error::NAN(e))?,
                z: coords[5].parse().map_err(|e| Error::NAN(e))?,
            },
        };

        steps.push((parts[0] == "on", cuboid));
    }

    Ok(steps)
}

fn main() -> Result<(), Error> {
    let steps = load_input(INPUT_FILE)?;
    let mut reactor = Reactor::new();
    let area = &Cuboid::new(Point::new(-50, -50, -50), Point::new(50, 50, 50));

    for step in &steps {
        reactor.merge_step(step.0, &step.1);
        if cfg!(debug_assertions) {
            println!("step:  on:{}  cuboid:{}", step.0, step.1);
            // println!("Reactor: {:?}", reactor);
            // println!("Cubes on: {}", reactor.count(&area));
            // println!();
        }
    }

    // println!("Reactor: {:?}", reactor);
    println!("Cubes on in {}: {}", area, reactor.count(&area));
    println!("Full Count: {}", reactor.full_count());

    Ok(())
}
