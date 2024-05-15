#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{BTreeSet, VecDeque};

pub struct Day20 {
    tiles: Vec<Vec<Tile>>,
    warps: Vec<Warp>,
    paths: Vec<Vec<Path>>,
}

struct Warp {
    c1: char,
    c2: char,
    inside: (usize, usize),
    outside: (usize, usize),
}

#[derive(Debug)]
struct Path {
    to: usize,
    steps: usize,
    from_inside: bool,
    to_inside: bool,
}

enum Tile {
    None,
    Wall,
    Hall,
    Letter(char),
    Warp { warp: usize, inside: bool },
}

impl Day20 {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            warps: vec![
                Warp {
                    c1: 'A',
                    c2: 'A',
                    inside: (0, 0),
                    outside: (0, 0),
                },
                Warp {
                    c1: 'Z',
                    c2: 'Z',
                    inside: (0, 0),
                    outside: (0, 0),
                },
            ],
            paths: Vec::new(),
        }
    }

    fn find_paths(&mut self) {
        for warp in 0..self.warps.len() {
            let mut paths = Vec::new();
            for (from_inside, coord) in [
                (false, self.warps[warp].outside),
                (true, self.warps[warp].inside),
            ] {
                if coord == (0, 0) {
                    continue;
                }

                let mut seen: BTreeSet<(usize, usize)> = BTreeSet::new();
                seen.insert(coord);
                let mut work = VecDeque::new();
                macro_rules! add_work {
                    ($steps:expr, $x:expr, $y:expr) => {
                        if seen.insert(($x, $y)) {
                            work.push_back(($steps, $x, $y));
                        }
                    };
                }
                add_work!(1, coord.0 + 1, coord.1);
                add_work!(1, coord.0 - 1, coord.1);
                add_work!(1, coord.0, coord.1 + 1);
                add_work!(1, coord.0, coord.1 - 1);

                while let Some((steps, x, y)) = work.pop_front() {
                    match &self.tiles[y][x] {
                        Tile::Hall => {
                            add_work!(steps + 1, x - 1, y);
                            add_work!(steps + 1, x + 1, y);
                            add_work!(steps + 1, x, y - 1);
                            add_work!(steps + 1, x, y + 1);
                        }
                        Tile::Warp { warp, inside } => {
                            paths.push(Path {
                                to: *warp,
                                steps,
                                from_inside,
                                to_inside: *inside,
                            });
                        }
                        _ => {}
                    }
                }
            }
            self.paths.push(paths);
        }
    }

    fn _print(&self) {
        for (idx, warp) in self.warps.iter().enumerate() {
            println!(
                "Warp {} is {}{}  Inside: {:?}  Outside: {:?}",
                idx, warp.c1, warp.c2, warp.inside, warp.outside
            );

            for path in self.paths[idx].iter() {
                println!("  {path:?}");
            }
        }
        for row in self.tiles.iter() {
            for tile in row.iter() {
                match tile {
                    Tile::Wall => print!("#"),
                    Tile::Warp { warp, inside } => print!(
                        "{}",
                        (*warp as u8 + if *inside { b'A' } else { b'a' }) as char
                    ),
                    _ => print!(" "),
                }
            }
            println!()
        }
    }

    fn solve(&self, recurse: bool) -> usize {
        #[derive(PartialOrd, Ord, PartialEq, Eq)]
        struct Work {
            steps: usize,
            at: usize,
            level: usize,
            inside: bool,
        }
        let mut work: BTreeSet<Work> = BTreeSet::new();
        work.insert(Work {
            steps: 0,
            at: 0,
            level: 0,
            inside: false,
        });

        while let Some(step) = work.pop_first() {
            if step.at == 1 {
                return step.steps;
            }
            for path in self.paths[step.at].iter() {
                if path.from_inside != step.inside {
                    continue;
                }
                if path.to == 1 && step.level == 0 {
                    let level = 0;
                    work.insert(Work {
                        steps: step.steps + path.steps,
                        at: path.to,
                        level,
                        inside: path.to_inside,
                    });
                }
                if path.to <= 1 {
                    continue;
                }
                if recurse && !path.to_inside && step.level == 0 {
                    continue;
                }

                let level = if !recurse {
                    step.level
                } else if path.to_inside {
                    step.level + 1
                } else {
                    step.level - 1
                };
                work.insert(Work {
                    steps: step.steps + path.steps + 1,
                    at: path.to,
                    level,
                    inside: !path.to_inside,
                });
            }
        }

        0
    }
}

impl Runner for Day20 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.tiles.push(
                line.chars()
                    .map(|c| match c {
                        ' ' => Tile::None,
                        '#' => Tile::Wall,
                        '.' => Tile::Hall,
                        'A'..='Z' => Tile::Letter(c),
                        _ => unreachable!(),
                    })
                    .collect(),
            );
        }

        macro_rules! add_warp {
            ($x:expr, $y:expr, $c1:expr, $c2:expr, $inside:expr) => {
                let warp =
                    if let Some(idx) = self.warps.iter().position(|w| w.c1 == $c1 && w.c2 == $c2) {
                        idx
                    } else {
                        self.warps.push(Warp {
                            c1: $c1,
                            c2: $c2,
                            inside: (0, 0),
                            outside: (0, 0),
                        });
                        self.warps.len() - 1
                    };
                if $inside {
                    self.warps[warp].inside = ($x, $y);
                } else {
                    self.warps[warp].outside = ($x, $y);
                }
                self.tiles[$y][$x] = Tile::Warp {
                    warp,
                    inside: $inside,
                };
            };
        }

        for y in 1..self.tiles.len() - 1 {
            for x in 1..self.tiles[y].len() - 1 {
                if let Tile::Letter(c1) = self.tiles[y][x] {
                    let inside = y != 1
                        && y != self.tiles.len() - 2
                        && x != 1
                        && x != self.tiles[y].len() - 2;

                    match (&self.tiles[y - 1][x], &self.tiles[y + 1][x]) {
                        (Tile::Letter(c2), Tile::Hall) => {
                            // Found label c2c1
                            add_warp!(x, y + 1, *c2, c1, inside);
                        }
                        (Tile::Hall, Tile::Letter(c2)) => {
                            // Found label c1c2
                            add_warp!(x, y - 1, c1, *c2, inside);
                        }
                        _ => {}
                    }
                    match (&self.tiles[y][x - 1], &self.tiles[y][x + 1]) {
                        (Tile::Letter(c2), Tile::Hall) => {
                            // Found label c2c1
                            add_warp!(x + 1, y, *c2, c1, inside);
                        }
                        (Tile::Hall, Tile::Letter(c2)) => {
                            // Found label c1c2
                            add_warp!(x - 1, y, c1, *c2, inside);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.find_paths();
        self._print();
        Ok(self.solve(false).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.find_paths();
        self._print();
        Ok(self.solve(true).into())
    }
}
