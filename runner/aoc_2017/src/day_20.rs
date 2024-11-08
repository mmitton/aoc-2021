#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Triple {
    x: isize,
    y: isize,
    z: isize,
}

impl Triple {
    fn distance_to(&self, rhs: &Self) -> usize {
        let x: usize = (self.x - rhs.x).unsigned_abs();
        let y: usize = (self.y - rhs.y).unsigned_abs();
        let z: usize = (self.z - rhs.z).unsigned_abs();
        x + y + z
    }
}

#[derive(Copy, Clone)]
struct Particle {
    p: Triple,
    v: Triple,
    a: Triple,
}

impl Particle {
    fn pos_at(&self, t: isize) -> Triple {
        Triple {
            x: (self.a.x * t * t / 2) + (self.v.x * t) + self.p.x,
            y: (self.a.y * t * t / 2) + (self.v.y * t) + self.p.y,
            z: (self.a.z * t * t / 2) + (self.v.z * t) + self.p.z,
        }
    }

    fn collides_with(&self, rhs: &Self) -> Option<usize> {
        #[derive(Debug)]
        enum CollisionAt {
            One(usize),
            Two(usize, usize),
            Always,
            Never,
        }

        impl CollisionAt {
            fn insert_times(&self, times: &mut Vec<usize>) {
                match self {
                    Self::One(t) => {
                        if !times.contains(t) {
                            times.push(*t);
                        }
                    }
                    Self::Two(t1, t2) => {
                        if !times.contains(t1) {
                            times.push(*t1);
                        }
                        if !times.contains(t2) {
                            times.push(*t2);
                        }
                    }
                    _ => {}
                }
            }

            fn matches(&self, t: usize) -> bool {
                match self {
                    Self::One(t1) => *t1 == t,
                    Self::Two(t1, t2) => *t1 == t || *t2 == t,
                    Self::Always => true,
                    Self::Never => false,
                }
            }
        }

        fn quadratic(a: isize, b: isize, c: isize) -> CollisionAt {
            if a == 0 {
                if b == 0 {
                    if c == 0 {
                        return CollisionAt::Always;
                    }
                    return CollisionAt::Never;
                } else {
                    let x = -c / b;
                    if -c % b == 0 && x >= 0 {
                        return CollisionAt::One(x as usize);
                    }
                    return CollisionAt::Never;
                }
            }

            let determinant = (b * b) - (4 * a * c);
            if determinant < 0 {
                return CollisionAt::Never;
            }

            let sqrt = (determinant as f64).sqrt() as isize;
            if sqrt * sqrt != determinant {
                return CollisionAt::Never;
            }

            let r1 = -b + sqrt;
            let r2 = -b - sqrt;

            if r1 * a >= 0 && r1 % (2 * a) == 0 {
                if r2 * a >= 0 && r2 % (2 * a) == 0 {
                    if r1 == r2 {
                        return CollisionAt::One((r1 / (2 * a)) as usize);
                    } else {
                        return CollisionAt::Two((r1 / (2 * a)) as usize, (r2 / (2 * a)) as usize);
                    }
                } else {
                    return CollisionAt::One((r1 / (2 * a)) as usize);
                }
            } else if r2 * a >= 0 && r2 % (2 * a) == 0 {
                return CollisionAt::One((r2 / (2 * a)) as usize);
            }

            CollisionAt::Never
        }

        let tx = quadratic(
            self.a.x - rhs.a.x,
            (2 * self.v.x + self.a.x) - (2 * rhs.v.x + rhs.a.x),
            (2 * self.p.x) - (2 * rhs.p.x),
        );
        let ty = quadratic(
            self.a.y - rhs.a.y,
            (2 * self.v.y + self.a.y) - (2 * rhs.v.y + rhs.a.y),
            (2 * self.p.y) - (2 * rhs.p.y),
        );
        let tz = quadratic(
            self.a.z - rhs.a.z,
            (2 * self.v.z + self.a.z) - (2 * rhs.v.z + rhs.a.z),
            (2 * self.p.z) - (2 * rhs.p.z),
        );

        let mut times = Vec::new();
        tx.insert_times(&mut times);
        ty.insert_times(&mut times);
        tz.insert_times(&mut times);

        times
            .into_iter()
            .find(|&time| tx.matches(time) && ty.matches(time) && tz.matches(time))
    }
}

impl FromStr for Particle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .replace("<", "")
            .replace(">", "")
            .replace("p=", "")
            .replace(", v=", " ")
            .replace(", a=", " ")
            .replace(",", " ");

        let parts: Vec<&str> = s.split(" ").collect();
        let p = Triple {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            z: parts[2].parse()?,
        };
        let v = Triple {
            x: parts[3].parse()?,
            y: parts[4].parse()?,
            z: parts[5].parse()?,
        };
        let a = Triple {
            x: parts[6].parse()?,
            y: parts[7].parse()?,
            z: parts[8].parse()?,
        };
        Ok(Particle { p, v, a })
    }
}

#[derive(Default)]
pub struct Day20 {
    particles: Vec<Particle>,
}

impl Day20 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day20 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.particles.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let origin = Triple { x: 0, y: 0, z: 0 };
        Ok(self
            .particles
            .iter()
            .enumerate()
            .fold((usize::MAX, usize::MAX), |closest, (i, p)| {
                let dist = p.pos_at(1_000_000).distance_to(&origin);
                if dist < closest.0 {
                    (dist, i)
                } else {
                    closest
                }
            })
            .1
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut collisions: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::default();
        let mut live = vec![true; self.particles.len()];

        for i in 0..self.particles.len() {
            for j in i + 1..self.particles.len() {
                if let Some(t) = self.particles[i].collides_with(&self.particles[j]) {
                    collisions.entry(t).or_default().push((i, j));
                }
            }
        }

        for collisions in collisions.values() {
            let mut remove = Vec::new();
            for (i, j) in collisions {
                if !live[*i] || !live[*j] {
                    continue;
                }
                remove.push(*i);
                remove.push(*j);
            }

            for i in &remove {
                live[*i] = false;
            }
        }

        Ok(live.iter().filter(|l| **l).count().into())
    }
}
