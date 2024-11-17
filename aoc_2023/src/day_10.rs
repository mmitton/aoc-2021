use std::collections::{BTreeMap, BTreeSet};

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn delta(&self, dx: isize, dy: isize) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Tile {
    a: Point,
    b: Point,
}

pub struct Day10 {
    starting: Point,
    tiles: BTreeMap<Point, Tile>,
}

impl Day10 {
    pub fn new() -> Self {
        Self {
            starting: Point::default(),
            tiles: BTreeMap::new(),
        }
    }

    fn get_loop(&self) -> Vec<Point> {
        let mut on = Vec::new();
        macro_rules! starting {
            ($p:expr) => {
                if let Some(t) = self.tiles.get(&$p) {
                    if t.a == self.starting || t.b == self.starting {
                        on.push(($p, self.starting));
                    }
                }
            };
        }
        starting!(self.starting.delta(-1, 0));
        starting!(self.starting.delta(1, 0));
        starting!(self.starting.delta(0, -1));
        starting!(self.starting.delta(0, 1));

        println!("starting: {:?}", self.starting);
        println!("on: {on:?}");
        assert_eq!(on.len(), 2);
        let mut path = vec![self.starting];
        loop {
            if on[0].0 == on[1].0 {
                path.push(on[0].0);
                break;
            }

            path.push(on[0].0);
            path.push(on[1].0);
            macro_rules! make_step {
                ($p:expr) => {{
                    let t = self.tiles.get(&$p.0).unwrap();
                    let np = if t.a == $p.1 { t.b } else { t.a };
                    $p.1 = $p.0;
                    $p.0 = np;
                }};
            }

            make_step!(on[0]);
            make_step!(on[1]);
        }
        path
    }
}

impl Runner for Day10 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        for (y, line) in Lines::from_bufread(file, LinesOpt::RAW)?.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = Point::new(x as isize, y as isize);
                match c {
                    '|' => {
                        let _ = self.tiles.insert(
                            p,
                            Tile {
                                a: p.delta(0, -1),
                                b: p.delta(0, 1),
                            },
                        );
                    }
                    '-' => {
                        let _ = self.tiles.insert(
                            p,
                            Tile {
                                a: p.delta(-1, 0),
                                b: p.delta(1, 0),
                            },
                        );
                    }
                    'L' => {
                        let _ = self.tiles.insert(
                            p,
                            Tile {
                                a: p.delta(0, -1),
                                b: p.delta(1, 0),
                            },
                        );
                    }
                    'J' => {
                        let _ = self.tiles.insert(
                            p,
                            Tile {
                                a: p.delta(0, -1),
                                b: p.delta(-1, 0),
                            },
                        );
                    }
                    '7' => {
                        let _ = self.tiles.insert(
                            p,
                            Tile {
                                a: p.delta(0, 1),
                                b: p.delta(-1, 0),
                            },
                        );
                    }
                    'F' => {
                        let _ = self.tiles.insert(
                            p,
                            Tile {
                                a: p.delta(0, 1),
                                b: p.delta(1, 0),
                            },
                        );
                    }
                    '.' => {}
                    'S' => self.starting = p,
                    _ => unreachable!("What is this map? '{c}'"),
                }
            }
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day10 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let path = self.get_loop();
        println!("{} {path:?}", path.len());

        Ok((path.len() / 2).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut path: BTreeSet<Point> = BTreeSet::new();
        path.extend(self.get_loop().as_slice());
        let x_vals: Vec<_> = path.iter().map(|p| p.x).collect();
        let y_vals: Vec<_> = path.iter().map(|p| p.x).collect();
        let x_min = *x_vals.iter().min().unwrap();
        let x_max = *x_vals.iter().max().unwrap();
        let y_min = *y_vals.iter().min().unwrap();
        let y_max = *y_vals.iter().max().unwrap();

        println!("x: {x_min}..={x_max}");
        println!("y: {y_min}..={y_max}");

        let mut inside = 0;
        for y in y_min..=y_max {
            let mut crosses = 0;
            let mut last_y = None;
            for x in x_min..=x_max {
                let p = Point::new(x, y);
                if path.contains(&p) {
                    if let Some(t) = self.tiles.get(&p) {
                        if t.a.y == p.y && t.b.y == p.y {
                            continue;
                        }

                        if t.a.y != p.y && t.b.y != p.y {
                            crosses += 1;
                        } else {
                            let next_y = if t.a.y != p.y { t.a.y } else { t.b.y };

                            if let Some(last_y) = last_y.take() {
                                if last_y != next_y {
                                    crosses += 1;
                                }
                            } else {
                                last_y = Some(next_y);
                            }
                        }
                    }
                    continue;
                } else if crosses % 2 == 1 {
                    inside += 1;
                }
            }
        }

        Ok(inside.into())
    }
}
