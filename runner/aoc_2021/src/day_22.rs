#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Cuboid {
    start: Point,
    end: Point,
}

impl Cuboid {
    fn new(start: Point, end: Point) -> Self {
        assert!(start.x <= end.x);
        assert!(start.y <= end.y);
        assert!(start.z <= end.z);
        Self { start, end }
    }

    fn clamp(&self, min: &Point, max: &Point) -> Option<Self> {
        let start = Point::new(
            self.start.x.max(min.x),
            self.start.y.max(min.y),
            self.start.z.max(min.z),
        );
        let end = Point::new(
            self.end.x.min(max.x),
            self.end.y.min(max.y),
            self.end.z.min(max.z),
        );

        if start.x <= end.x && start.y <= end.y && start.z <= end.z {
            Some(Cuboid { start, end })
        } else {
            None
        }
    }

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

        // Top
        if rhs_start_y != self.start.y {
            non_overlaps.push(Cuboid::new(
                Point::new(rhs_start_x, self.start.y, rhs_start_z),
                Point::new(rhs_end_x, rhs_start_y - 1, rhs_end_z),
            ));
        }

        // Left
        if rhs_start_x != self.start.x {
            non_overlaps.push(Cuboid::new(
                Point::new(self.start.x, self.start.y, rhs_start_z),
                Point::new(rhs_start_x - 1, self.end.y, rhs_end_z),
            ));
        }

        // Right
        if rhs_end_x != self.end.x {
            non_overlaps.push(Cuboid::new(
                Point::new(rhs_end_x + 1, self.start.y, rhs_start_z),
                Point::new(self.end.x, self.end.y, rhs_end_z),
            ));
        }

        // Bottom
        if rhs_end_y != self.end.y {
            non_overlaps.push(Cuboid::new(
                Point::new(rhs_start_x, rhs_end_y + 1, rhs_start_z),
                Point::new(rhs_end_x, self.end.y, rhs_end_z),
            ));
        }
        Some(non_overlaps)
    }
}

pub struct Day22 {
    cuboids: Vec<Cuboid>,
    limits: [Point; 2],
    steps: Vec<(bool, Cuboid)>,
}

impl Day22 {
    pub fn new() -> Self {
        Self {
            cuboids: Vec::new(),
            limits: [
                Point::new(isize::MIN, isize::MIN, isize::MIN),
                Point::new(isize::MAX, isize::MAX, isize::MAX),
            ],
            steps: Vec::new(),
        }
    }

    fn count(&self) -> usize {
        let mut on = 0usize;
        for c in &self.cuboids {
            on += (c.end.x - c.start.x + 1) as usize
                * (c.end.y - c.start.y + 1) as usize
                * (c.end.z - c.start.z + 1) as usize;
        }

        on
    }

    fn merge(&mut self) {
        for (turn_on, cuboid) in self.steps.iter() {
            if let Some(cuboid) = cuboid.clamp(&self.limits[0], &self.limits[1]) {
                // For both turn on and turn off, remove cuboids which overlap and add back in parts which
                // do not as new cuboids
                let mut to_add = Vec::new();
                self.cuboids.retain(|c| {
                    if let Some(non_overlaps) = c.overlaps(&cuboid) {
                        to_add.extend_from_slice(&non_overlaps);
                        false
                    } else {
                        true
                    }
                });
                self.cuboids.extend_from_slice(&to_add);

                // Finally, if this is a turn on, add it to the list
                if *turn_on {
                    // Turn On
                    self.cuboids.push(cuboid.clone());
                }
            }
        }
    }
}

impl Runner for Day22 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        for line in lines.iter() {
            let parts: Vec<&str> = line.split(" ").collect();
            let coords = parts[1].replace("x=", "");
            let coords = coords.replace(",y=", " ");
            let coords = coords.replace(",z=", " ");
            let coords = coords.replace("..", " ");
            let coords: Vec<&str> = coords.split(" ").collect();

            let cuboid = Cuboid {
                start: Point {
                    x: coords[0].parse()?,
                    y: coords[2].parse()?,
                    z: coords[4].parse()?,
                },
                end: Point {
                    x: coords[1].parse()?,
                    y: coords[3].parse()?,
                    z: coords[5].parse()?,
                },
            };

            self.steps.push((parts[0] == "on", cuboid));
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.limits[0] = Point::new(-50, -50, -50);
        self.limits[1] = Point::new(50, 50, 50);
        self.merge();
        Ok(self.count().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.merge();
        Ok(self.count().into())
    }
}
