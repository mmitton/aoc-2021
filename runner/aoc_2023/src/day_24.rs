#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Triple {
    x: i128,
    y: i128,
    z: i128,
}

impl Triple {
    fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn dot(&self, other: Self) -> i128 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn mul(&self, n: i128) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }

    fn div(&self, n: i128) -> Self {
        Self {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

impl FromStr for Triple {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        if parts.len() != 3 {
            return Err(Error::InvalidInput(format!("Triple: '{s}'")));
        }

        let x = parts[0].trim().parse()?;
        let y = parts[1].trim().parse()?;
        let z = parts[2].trim().parse()?;

        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Hailstone {
    p: Triple,
    v: Triple,
}

impl Hailstone {
    fn crosses_xy(&self, rhs: &Self) -> Option<(f64, f64, f64, f64)> {
        // slope = Rise/Run
        let a: f64 = self.v.y as f64 / self.v.x as f64;
        let b: f64 = rhs.v.y as f64 / rhs.v.x as f64;
        // y intersept =
        // x = px + t*vx
        // y = py + t*vy
        // (x - px) / vx = t
        // (y - py) / vy = t
        // (x - px) / vx = (y - py) / vy
        // (-px * vy) / vx = y - py
        // ((-px * vy) / vx) + py = y
        let c: f64 = ((-self.p.x as f64 * self.v.y as f64) / self.v.x as f64) + self.p.y as f64;
        let d: f64 = ((-rhs.p.x as f64 * rhs.v.y as f64) / rhs.v.x as f64) + rhs.p.y as f64;

        if a - b == 0.0 {
            return None;
        }

        let x = (d - c) / (a - b);
        let y = (a * x) + c;

        let t1 = (x - self.p.x as f64) / self.v.x as f64;
        let t2 = (x - rhs.p.x as f64) / rhs.v.x as f64;

        Some((x, y, t1, t2))
    }
}

impl FromStr for Hailstone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((p, v)) = s.split_once(" @ ") {
            Ok(Self {
                p: p.parse()?,
                v: v.parse()?,
            })
        } else {
            Err(Error::InvalidInput(format!("Hailstone: '{s}'")))
        }
    }
}

pub struct Day24 {
    hailstones: Vec<Hailstone>,
}

impl Day24 {
    pub fn new() -> Self {
        Self {
            hailstones: Vec::new(),
        }
    }
}

impl Runner for Day24 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        for line in Lines::from_path(path, LinesOpt::RAW)?.iter() {
            self.hailstones.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let testarea = if self.hailstones.len() == 5 {
            7.0..=27.0
        } else {
            200000000000000.0..=400000000000000.0
        };
        let mut ans = 0;
        for (i, a) in self.hailstones.iter().enumerate() {
            for b in self.hailstones.iter().skip(i + 1) {
                if let Some((x, y, ta, tb)) = a.crosses_xy(b) {
                    if ta > 0. && tb > 0. && testarea.contains(&x) && testarea.contains(&y) {
                        ans += 1;
                    }
                }
            }
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let h0 = self.hailstones[0];

        let transformed = self
            .hailstones
            .iter()
            .take(4)
            .map(|h| Hailstone {
                p: h.p.sub(h0.p),
                v: h.v.sub(h0.v),
            })
            .collect::<Vec<_>>();

        // h1 defines a line that the target stone must pass through. Define a plane
        // using the origin and that line.
        let h1 = transformed[1];
        let normal = h1.p.cross(h1.v);

        // calculate the time and location of the intersection of h2 with
        // the plane defined by the line from the origin to h1 and normal.
        let h2 = transformed[2];
        let t2 = h1.p.sub(h2.p).dot(normal) / h2.v.dot(normal);
        let h2_intersection = h2.p.add(h2.v.mul(t2));

        // and same for h3
        let h3 = transformed[3];
        let t3 = h1.p.sub(h3.p).dot(normal) / h3.v.dot(normal);
        let h3_intersection = h3.p.add(h3.v.mul(t3));

        let rock_vel = h3_intersection.sub(h2_intersection).div(t3 - t2);
        let rock_pos = h3_intersection.add(rock_vel.mul(-t3));
        let rock = Hailstone {
            p: rock_pos,
            v: rock_vel,
        };

        // retransform rock back to the starting coordinate system
        let rock = Hailstone {
            p: rock.p.add(h0.p),
            v: rock.v.add(h0.v),
        };
        let sum = (rock.p.x + rock.p.y + rock.p.z) as usize;

        Ok(sum.into())
    }
}
