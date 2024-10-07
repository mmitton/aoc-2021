#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day13 {
    points: Vec<(usize, usize)>,
    folds: Vec<(char, usize)>,
    max_x: usize,
    max_y: usize,
}

impl Day13 {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            folds: Vec::new(),
            max_x: 0,
            max_y: 0,
        }
    }

    fn display(&self) -> String {
        let mut display = String::new();
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                if self.points.contains(&(x, y)) {
                    display.push('#');
                } else {
                    display.push(' ');
                }
            }
            display.push('\n');
        }
        display
    }

    fn fold(&mut self, once: bool) {
        let mut points = HashSet::default();
        for fold in self.folds.iter() {
            match fold.0 {
                'x' => {
                    self.max_x = fold.1 - 1;

                    points.clear();
                    let sub = fold.1 * 2;
                    for mut point in self.points.drain(..) {
                        if point.0 > fold.1 {
                            point.0 = sub - point.0;
                        }
                        points.insert(point);
                    }
                    for point in points.iter().copied() {
                        self.points.push(point);
                    }
                }
                'y' => {
                    self.max_y = fold.1 - 1;

                    points.clear();
                    let sub = fold.1 * 2;
                    for mut point in self.points.drain(..) {
                        if point.1 > fold.1 {
                            point.1 = sub - point.1;
                        }
                        points.insert(point);
                    }
                    for point in points.iter().copied() {
                        self.points.push(point);
                    }
                }
                _ => unreachable!(),
            }
            if once {
                break;
            }
        }
    }
}

impl Runner for Day13 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();

        // Parse points
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let (x, y) = line.split_once(',').expect("Invalid point");
            let x: usize = x.parse().expect("Invalid x coord: {line:?}");
            let y: usize = y.parse().expect("Invalid y coord: {line:?}");

            self.max_x = self.max_x.max(x);
            self.max_y = self.max_y.max(y);

            self.points.push((x, y));
        }

        // Parse folds
        for line in lines {
            let (axis, at) = line.split_once("=").expect("Invalid fold");
            self.folds.push((
                if axis.ends_with('x') { 'x' } else { 'y' },
                at.parse().expect("Invalid fold: {line:?}"),
            ));
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.fold(true);
        Ok(self.points.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.fold(false);
        Ok(self.display().into())
    }
}
