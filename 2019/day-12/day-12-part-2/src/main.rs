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

#[derive(Clone, Debug)]
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

    let initial_moons: Vec<Moon> = moons.to_vec();
    let mut x_steps = None;
    let mut y_steps = None;
    let mut z_steps = None;
    let mut steps = 0;
    while x_steps.is_none() || y_steps.is_none() || z_steps.is_none() {
        steps += 1;
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

        macro_rules! check_loop {
            ($axis:ident, $steps:ident) => {
                if $steps.is_none() {
                    let mut initial = true;
                    for (moon, initial_moon) in moons.iter().zip(initial_moons.iter()) {
                        if moon.pos.$axis != initial_moon.pos.$axis
                            && moon.vel.$axis != initial_moon.vel.$axis
                        {
                            initial = false;
                            break;
                        }
                    }

                    if initial {
                        $steps = Some(steps);
                    }
                }
            };
        }

        check_loop!(x, x_steps);
        check_loop!(y, y_steps);
        check_loop!(z, z_steps);
    }

    let mut axis_steps = vec![x_steps.unwrap(), y_steps.unwrap(), z_steps.unwrap()];
    axis_steps.sort();

    let mut min_steps = axis_steps[2];
    loop {
        if min_steps % axis_steps[1] == 0 {
            break;
        }
        min_steps += axis_steps[2];
    }

    let mut steps = 0u128;
    loop {
        steps += min_steps;
        if steps % axis_steps[0] == 0 {
            println!("ans: {}", steps * 2);
            break;
        }
    }
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
