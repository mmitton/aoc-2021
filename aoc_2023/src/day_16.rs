#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone)]
enum Optics {
    Empty {
        energized: bool,
    },
    Mirror {
        typ: char,
        e1: bool,
        e2: bool,
    },
    Splitter {
        typ: char,
        energized: bool,
        split: bool,
    },
}

#[derive(Clone)]
pub struct Day16 {
    tiles: Vec<Vec<Optics>>,
}

impl Day16 {
    pub fn new() -> Self {
        Self { tiles: Vec::new() }
    }

    fn _dump(&self, energized: bool) {
        for row in self.tiles.iter() {
            for col in row.iter() {
                print!(
                    "{}",
                    if energized {
                        match col {
                            Optics::Empty { energized: true }
                            | Optics::Mirror {
                                typ: _,
                                e1: true,
                                e2: _,
                            }
                            | Optics::Mirror {
                                typ: _,
                                e1: _,
                                e2: true,
                            }
                            | Optics::Splitter {
                                typ: _,
                                energized: true,
                                split: _,
                            } => '#',
                            _ => '.',
                        }
                    } else {
                        match col {
                            Optics::Empty { energized: _ } => '.',
                            Optics::Mirror {
                                typ: c,
                                e1: _,
                                e2: _,
                            }
                            | Optics::Splitter {
                                typ: c,
                                energized: _,
                                split: _,
                            } => *c,
                        }
                    }
                );
            }
            println!();
        }
    }

    fn energize(&mut self, mut x: usize, mut y: usize, dx: isize, dy: isize) {
        loop {
            if let Some(row) = self.tiles.get_mut(y) {
                if let Some(tile) = row.get_mut(x) {
                    match tile {
                        Optics::Empty { energized } => *energized = true,
                        Optics::Mirror { typ: '/', e1, e2 } => {
                            if dx == 1 || dy == 1 {
                                if *e1 {
                                    return;
                                }
                                *e1 = true;
                            } else {
                                if *e2 {
                                    return;
                                }
                                *e2 = true;
                            }
                            x = (x as isize - dy) as usize;
                            y = (y as isize - dx) as usize;
                            self.energize(x, y, -dy, -dx);
                            return;
                        }
                        Optics::Mirror { typ: '\\', e1, e2 } => {
                            if dx == 1 || dy == -1 {
                                if *e1 {
                                    return;
                                }
                                *e1 = true;
                            } else {
                                if *e2 {
                                    return;
                                }
                                *e2 = true;
                            }
                            x = (x as isize + dy) as usize;
                            y = (y as isize + dx) as usize;
                            self.energize(x, y, dy, dx);
                            return;
                        }
                        Optics::Splitter {
                            typ: '-',
                            energized,
                            split,
                        } => {
                            *energized = true;
                            if dx == 0 {
                                if !*split {
                                    *split = true;
                                    self.energize(x, y, -1, 0);
                                    self.energize(x, y, 1, 0);
                                }
                                return;
                            }
                        }
                        Optics::Splitter {
                            typ: '|',
                            energized,
                            split,
                        } => {
                            *energized = true;
                            if dy == 0 {
                                if !*split {
                                    *split = true;
                                    self.energize(x, y, 0, -1);
                                    self.energize(x, y, 0, 1);
                                }
                                return;
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    return;
                }
            } else {
                return;
            }
            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;
        }
    }

    fn count_energized(&self) -> usize {
        let mut ans = 0;
        for row in self.tiles.iter() {
            for col in row.iter() {
                match col {
                    Optics::Empty { energized: true }
                    | Optics::Mirror {
                        typ: _,
                        e1: true,
                        e2: _,
                    }
                    | Optics::Mirror {
                        typ: _,
                        e1: _,
                        e2: true,
                    }
                    | Optics::Splitter {
                        typ: _,
                        energized: true,
                        split: _,
                    } => ans += 1,
                    _ => {}
                }
            }
        }
        ans
    }
}

impl Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
            self.tiles.push(
                line.chars()
                    .map(|c| match c {
                        '/' | '\\' => Optics::Mirror {
                            typ: c,
                            e1: false,
                            e2: false,
                        },
                        '|' | '-' => Optics::Splitter {
                            typ: c,
                            energized: false,
                            split: false,
                        },
                        '.' => Optics::Empty { energized: false },
                        _ => unreachable!(),
                    })
                    .collect(),
            );
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

impl Day16 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.energize(0, 0, 1, 0);

        self._dump(false);
        println!();
        self._dump(true);

        Ok(self.count_energized().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut best = 0;

        let width = self.tiles[0].len();
        let height = self.tiles.len();

        macro_rules! run {
            ($x:expr, $y:expr, $dx:expr, $dy:expr) => {{
                let mut run = self.clone();
                run.energize($x, $y, $dx, $dy);
                best = best.max(run.count_energized());
            }};
        }

        for x in 0..width {
            run!(x, 0, 0, 1);
            run!(x, height - 1, 0, -1);
        }
        for y in 0..height {
            run!(0, y, 1, 0);
            run!(width - 1, y, -1, 0);
        }
        Ok(best.into())
    }
}
