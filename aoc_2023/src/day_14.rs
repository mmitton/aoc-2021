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
    x: i8,
    y: i8,
}

impl Point {
    fn new(x: i8, y: i8) -> Self {
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

    fn add_rock(&mut self, x: i8, y: i8, rounded: bool) {
        self.rocks.push(Rock {
            point: Point::new(x, y),
            rounded,
        });
    }

    fn north_load(&self) -> isize {
        let mut total_load = 0;
        for rock in self.rocks.iter() {
            if rock.rounded {
                total_load += self.max.y as isize - rock.point.y as isize + 1;
            }
        }
        total_load
    }

    fn tilt(&mut self, dir: Dir) {
        // Tilt the dish in dir Direction.  First sort the rocks so we process all of the rocks in
        // order by distance from the edge we roll towards.  This lets us keep a cache of the next
        // open position for all of the rocks rolling in that direction, updating the list to the
        // rock we just processed and adding one in the opposite direction of the tilt.
        //
        // Rolling to the north, the list is all columns, starting at 0, updating to rock.point.y +
        // 1 so the next rock will tuck in right behind us.  Of course, only updating the position
        // of rolling rocks.  Cube rocks just update the cached next position to + 1 of the cubed
        // rock position.
        let (mut stop_at, dx, dy) = match dir {
            Dir::North => {
                // Sort north to south, open positions start in the north
                self.rocks.sort_by_key(|r| r.point.y);
                (vec![0; self.max.x as usize + 1], 0, 1)
            }
            Dir::South => {
                // Sort south to north, open positions start in the south
                self.rocks.sort_by_key(|r| self.max.y - r.point.y);
                (vec![self.max.y; self.max.x as usize + 1], 0, -1)
            }
            Dir::West => {
                // Sort west to east, open positions start in the west
                self.rocks.sort_by_key(|r| r.point.x);
                (vec![0; self.max.y as usize + 1], 1, 0)
            }
            Dir::East => {
                // Sort east to west, open positions start in the east
                self.rocks.sort_by_key(|r| self.max.x - r.point.x);
                (vec![self.max.x; self.max.y as usize + 1], -1, 0)
            }
        };

        // Process all of the rocks now that they are sorted (described above)
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

    fn rounded_rocks(&self) -> Vec<Point> {
        self.rocks
            .iter()
            .filter_map(|r| if r.rounded { Some(r.point) } else { None })
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
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        for (y, line) in Lines::from_bufread(file, LinesOpt::RAW)?.iter().enumerate() {
            let y = y as i8;
            self.dish.max.y = self.dish.max.y.max(y);
            for (x, c) in line.chars().enumerate() {
                let x = x as i8;
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
        const TOTAL_CYCLES: usize = 1000000000;

        self.dish._dump();

        let mut seen = HashMap::new();
        for i in 1..=TOTAL_CYCLES {
            self.dish.spin();
            println!("after {i} cycles");
            self.dish._dump();

            let rounded = self.dish.rounded_rocks();
            if let Some(cycle_start) = seen.insert(rounded, i) {
                let cycle_len = i - cycle_start;
                let spins_remaining = TOTAL_CYCLES - i;
                let spins_after_cycles = spins_remaining % cycle_len;
                println!("Found a cycle from {cycle_start} to {i}, {spins_remaining} spins remaining, {spins_after_cycles} spins after the cycles complete");
                for _ in 0..spins_after_cycles {
                    self.dish.spin();
                }

                break;
            }
        }

        Ok(self.dish.north_load().into())
    }
}
