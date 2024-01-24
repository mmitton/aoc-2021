#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Copy, Clone, Debug, Default)]
struct Tripple {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Copy, Clone, Debug, Default)]
struct Moon {
    pos: Tripple,
    vel: Tripple,
}

pub struct Day12 {
    moons: [Moon; 4],
}

impl Day12 {
    pub fn new() -> Self {
        Self {
            moons: [Moon::default(); 4],
        }
    }

    pub fn step(&mut self) {
        use std::cmp::Ordering;
        for i in 0..self.moons.len() {
            for j in i + 1..self.moons.len() {
                macro_rules! apply_gravity {
                    ($axis:ident) => {
                        match self.moons[i].pos.$axis.cmp(&self.moons[j].pos.$axis) {
                            Ordering::Less => {
                                self.moons[i].vel.$axis += 1;
                                self.moons[j].vel.$axis -= 1;
                            }
                            Ordering::Greater => {
                                self.moons[i].vel.$axis -= 1;
                                self.moons[j].vel.$axis += 1;
                            }
                            Ordering::Equal => {}
                        }
                    };
                }

                apply_gravity!(x);
                apply_gravity!(y);
                apply_gravity!(z);
            }
        }

        for moon in self.moons.iter_mut() {
            moon.pos.x += moon.vel.x;
            moon.pos.y += moon.vel.y;
            moon.pos.z += moon.vel.z;
        }
    }
}

impl Runner for Day12 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 4);
        self.moons.iter_mut().zip(lines.iter()).for_each(|(m, l)| {
            let l = l.strip_suffix('>').unwrap();
            for (n, part) in l.split(", ").enumerate() {
                let (_, s) = part.split_once('=').unwrap();
                match n {
                    0 => m.pos.x = s.parse().unwrap(),
                    1 => m.pos.y = s.parse().unwrap(),
                    2 => m.pos.z = s.parse().unwrap(),
                    _ => unreachable!(),
                }
            }
        });
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        for moon in self.moons.iter() {
            println!("{moon:?}");
        }
        for _ in 0..1000 {
            self.step();
        }

        Ok(self
            .moons
            .iter()
            .map(|moon| {
                (moon.pos.x.unsigned_abs() + moon.pos.y.unsigned_abs() + moon.pos.z.unsigned_abs())
                    * (moon.vel.x.unsigned_abs()
                        + moon.vel.y.unsigned_abs()
                        + moon.vel.z.unsigned_abs())
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let initial = self.moons;
        let mut x = None;
        let mut y = None;
        let mut z = None;
        let mut found = 0;
        for i in 1.. {
            self.step();

            macro_rules! check_axis {
                ($axis:ident) => {
                    if $axis.is_none() {
                        let matches = self
                            .moons
                            .iter()
                            .zip(initial.iter())
                            .map(|(m, i)| {
                                if m.pos.$axis == i.pos.$axis && m.vel.$axis == i.vel.$axis {
                                    1
                                } else {
                                    0
                                }
                            })
                            .sum::<usize>();
                        if matches == 4 {
                            $axis = Some(i);
                            found += 1;
                        }
                    }
                };
            }

            check_axis!(x);
            check_axis!(y);
            check_axis!(z);

            if found == 3 {
                break;
            }
        }

        let x = x.unwrap();
        let y = y.unwrap();
        let z = z.unwrap();
        println!("{x} {y} {z}");
        Ok(helper::lcm(x, helper::lcm(y, z)).into())
    }
}
