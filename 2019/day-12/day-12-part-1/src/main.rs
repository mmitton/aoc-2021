const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
struct Triple {
    x: isize,
    y: isize,
    z: isize,
}

impl fmt::Debug for Triple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={:>5}, y={:>5}, z={:>5}>", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Moon {
    pos: Triple,
    vel: Triple,
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self {
            pos: Triple { x, y, z },
            vel: Triple { x: 0, y: 0, z: 0 },
        }
    }

    fn apply_gravity(&mut self, other: Triple) {
        macro_rules! apply {
            ($axis:ident) => {{
                if self.pos.$axis < other.$axis {
                    self.vel.$axis += 1;
                } else if self.pos.$axis > other.$axis {
                    self.vel.$axis -= 1;
                }
            }};
        }

        apply!(x);
        apply!(y);
        apply!(z);
    }
}

fn process(moons: &mut [Moon]) {
    if moons.is_empty() {
        return;
    }

    macro_rules! print_step {
        ($step:expr) => {{
            println!("After {} steps:", $step);
            let mut energy = 0;
            for moon in moons.iter() {
                println!("pos={:?}, vel={:?}", moon.pos, moon.vel);
                energy += (moon.pos.x.abs() + moon.pos.y.abs() + moon.pos.z.abs())
                    * (moon.vel.x.abs() + moon.vel.y.abs() + moon.vel.z.abs());
            }
            println!("Sum of total energy: {energy}");
        }};
    }

    print_step!(0);

    let total_steps = if cfg!(debug_assertions) { 100 } else { 1000 };
    for step in 1..=total_steps {
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }
                moons[i].apply_gravity(moons[j].pos);
            }
        }
        for moon in moons.iter_mut() {
            moon.pos.x += moon.vel.x;
            moon.pos.y += moon.vel.y;
            moon.pos.z += moon.vel.z;
        }

        if cfg!(debug_assertions) && step != total_steps {
            print_step!(step);
        }
    }
    print_step!(total_steps);
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut moons: Vec<Moon> = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            process(&mut moons);
            moons.clear();
            continue;
        }

        let line = &line[1..line.len() - 1];
        let line = line
            .replace("x=", "")
            .replace("y=", "")
            .replace("z=", "")
            .replace(' ', "");
        let parts: Vec<&str> = line.split(',').collect();
        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();
        let z = parts[2].parse().unwrap();
        moons.push(Moon::new(x, y, z));
    }
    process(&mut moons);
}
