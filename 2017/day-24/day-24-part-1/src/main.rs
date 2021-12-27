#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

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

impl Triple {
    fn distance_to(&self, rhs: &Self) -> usize {
        let x: usize = (self.x - rhs.x).abs() as usize;
        let y: usize = (self.y - rhs.y).abs() as usize;
        let z: usize = (self.z - rhs.z).abs() as usize;
        x + y + z
    }
}

#[derive(Debug)]
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
        Ok(Particle {
            p: Triple {
                x: parts[0].parse()?,
                y: parts[1].parse()?,
                z: parts[2].parse()?,
            },
            v: Triple {
                x: parts[3].parse()?,
                y: parts[4].parse()?,
                z: parts[5].parse()?,
            },
            a: Triple {
                x: parts[6].parse()?,
                y: parts[7].parse()?,
                z: parts[8].parse()?,
            },
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
    let origin = Triple { x: 0, y: 0, z: 0 };

    let mut closest_num = usize::MAX;
    let mut closest = usize::MAX;

    const T: isize = 1000000;
    for (i, particle) in particles.iter().enumerate() {
        let dist = particle.pos_at(T).distance_to(&origin);
        println!("{}: {}", i, dist);

        if dist < closest {
            closest = dist;
            closest_num = i;
        }
    }

    println!("Closest is {}", closest_num);

    Ok(())
}
