#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Triple {
    x: isize,
    y: isize,
    z: isize,
}

impl std::fmt::Debug for Triple {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Clone, Debug)]
struct Particle {
    p: Triple,
    v: Triple,
    a: Triple,
}

impl Particle {
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
            } else {
                if r2 * a >= 0 && r2 % (2 * a) == 0 {
                    return CollisionAt::One((r2 / (2 * a)) as usize);
                }
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

        for time in times {
            if tx.matches(time) && ty.matches(time) && tz.matches(time) {
                return Some(time);
            }
        }

        None
    }
}

impl TryFrom<&str> for Particle {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
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
        Ok(Particle {
            p: p.clone(),
            v: v,
            a: a,
        })
    }
}

fn load_input(filename: &str) -> Result<Vec<Particle>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut particles: Vec<Particle> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        particles.push(line.try_into()?);
    }

    Ok(particles)
}

fn main() -> Result<(), Error> {
    let particles = load_input(INPUT_FILE)?;

    let mut collisions: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
    let mut live = vec![true; particles.len()];

    for i in 0..particles.len() {
        for j in i + 1..particles.len() {
            if let Some(t) = particles[i].collides_with(&particles[j]) {
                if !collisions.contains_key(&t) {
                    collisions.insert(t, Vec::new());
                }
                collisions.get_mut(&t).unwrap().push((i, j));
            }
        }
    }

    for t in collisions.keys() {
        println!("Processing collisions at {}", t,);
        let collisions = collisions.get(&t).unwrap();
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

    let mut live_count = 0usize;
    for is_live in &live {
        if *is_live {
            live_count += 1;
        }
    }

    println!("Number left is : {}", live_count);
    Ok(())
}
