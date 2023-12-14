#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::HashMap;

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

struct Rock {
    point: Point,
    rounded: bool,
}

struct Dish {
    rocks: Vec<Rock>,
    max: Point,
}

enum Dir {
    North,
    South,
    East,
    West,
}

impl Dish {
    fn new() -> Self {
        Self {
            rocks: Vec::new(),
            max: Point::new(0, 0),
        }
    }

    fn _dump(&self) {
        let mut map = vec![vec!['.'; self.max.x as usize + 1]; self.max.y as usize + 1];
        for rock in self.rocks.iter() {
            map[rock.point.y as usize][rock.point.x as usize] = match rock.rounded {
                true => 'O',
                false => '#',
            };
        }
        for row in map.iter() {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }

    fn add_rock(&mut self, x: isize, y: isize, rounded: bool) {
        self.rocks.push(Rock {
            point: Point::new(x, y),
            rounded,
        });
    }

    fn north_load(&self) -> isize {
        let mut total_load = 0;
        for rock in self.rocks.iter() {
            if rock.rounded {
                total_load += self.max.y - rock.point.y + 1;
            }
        }
        total_load
    }

    fn tilt(&mut self, dir: Dir) {
        let (mut stop_at, dx, dy) = match dir {
            Dir::North => {
                self.rocks.sort_by_key(|r| r.point.y);
                (vec![0; self.max.x as usize + 1], 0, 1)
            }
            Dir::South => {
                self.rocks.sort_by_key(|r| self.max.y - r.point.y);
                (vec![self.max.y; self.max.x as usize + 1], 0, -1)
            }
            Dir::West => {
                self.rocks.sort_by_key(|r| r.point.x);
                (vec![0; self.max.y as usize + 1], 1, 0)
            }
            Dir::East => {
                self.rocks.sort_by_key(|r| self.max.x - r.point.x);
                (vec![self.max.x; self.max.y as usize + 1], -1, 0)
            }
        };

        for rock in self.rocks.iter_mut() {
            match dir {
                Dir::North | Dir::South => {
                    if rock.rounded {
                        rock.point.y = stop_at[rock.point.x as usize];
                    }
                    stop_at[rock.point.x as usize] = rock.point.y + dy;
                }
                Dir::West | Dir::East => {
                    if rock.rounded {
                        rock.point.x = stop_at[rock.point.y as usize];
                    }
                    stop_at[rock.point.y as usize] = rock.point.x + dx;
                }
            }
        }
    }

    fn spin(&mut self) {
        self.tilt(Dir::North);
        self.tilt(Dir::West);
        self.tilt(Dir::South);
        self.tilt(Dir::East);
    }

    fn rounded_vec(&self) -> Vec<u16> {
        self.rocks
            .iter()
            .filter_map(|r| {
                if r.rounded {
                    Some(((r.point.y as u16) << 8) | (r.point.x as u16))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct Day14 {
    dish: Dish,
}

impl Day14 {
    pub fn new() -> Self {
        Self { dish: Dish::new() }
    }
}

impl Runner for Day14 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        for (y, line) in Lines::from_path(path, LinesOpt::RAW)?.iter().enumerate() {
            let y = y as isize;
            self.dish.max.y = self.dish.max.y.max(y);
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                self.dish.max.x = self.dish.max.x.max(x);
                match c {
                    '.' => {}
                    '#' => self.dish.add_rock(x, y, false),
                    'O' => self.dish.add_rock(x, y, true),
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.dish._dump();
        self.dish.tilt(Dir::North);
        self.dish._dump();

        Ok(self.dish.north_load().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.dish._dump();
        let mut seen = HashMap::new();
        let total_cycles = 1000000000;
        for i in 1..=total_cycles {
            self.dish.spin();
            println!("after {i} cycles");
            self.dish._dump();

            let rounded = self.dish.rounded_vec();
            if let Some(cycle_start) = seen.insert(rounded, i) {
                let cycle_len = i - cycle_start;
                let remaining = total_cycles - i;
                let mod_cycles = remaining % cycle_len;
                println!("Found a cycle from {cycle_start} to {i}, {remaining} cycles, {mod_cycles} mod_cycles");
                for _ in 0..mod_cycles {
                    self.dish.spin();
                }

                break;
            }
        }

        Ok(self.dish.north_load().into())
    }
}
