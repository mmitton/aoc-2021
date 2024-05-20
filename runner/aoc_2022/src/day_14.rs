#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day14 {
    grid: Vec<Vec<bool>>,
    x: (i16, i16),
    y: (i16, i16),
}

impl Day14 {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(),
            x: (i16::MAX, i16::MIN),
            y: (i16::MAX, i16::MIN),
        }
    }

    fn solve<const HAS_FLOOR: bool>(&mut self) -> usize {
        let mut ans = 0;
        'run_loop: loop {
            ans += 1;
            let mut sand = ((500 - self.x.0) as usize, 0);
            'fall_loop: loop {
                if sand.1 == self.grid.len() - 1 {
                    if HAS_FLOOR {
                        self.grid[sand.1][sand.0] = true;
                        break 'fall_loop;
                    } else {
                        ans -= 1;
                        break 'run_loop;
                    }
                }

                if self.grid[sand.1 + 1][sand.0] {
                    // Something is below, scan left
                    if self.grid[sand.1 + 1][sand.0 - 1] {
                        // something is to the left
                        if self.grid[sand.1 + 1][sand.0 + 1] {
                            // Something is to the right, stay here
                            if sand.1 == 0 {
                                break 'run_loop;
                            }
                            self.grid[sand.1][sand.0] = true;
                            break 'fall_loop;
                        } else {
                            sand.0 += 1;
                            sand.1 += 1;
                        }
                    } else {
                        sand.0 -= 1;
                        sand.1 += 1;
                    }
                } else {
                    sand.1 += 1;
                }
            }
            // self.x.0 = self.x.0.min(sand.0);
            // self.x.1 = self.x.1.max(sand.1);
        }
        ans
    }
}

impl Runner for Day14 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        let mut coords: Vec<Vec<(i16, i16)>> = Vec::new();
        for line in lines.iter() {
            coords.push(
                line.split(" -> ")
                    .map(|c| {
                        let (x, y) = c.split_once(',').unwrap();
                        let x: i16 = x.parse().unwrap();
                        let y: i16 = y.parse().unwrap();

                        self.x.0 = self.x.0.min(x);
                        self.y.0 = self.y.0.min(y);
                        self.x.1 = self.x.1.max(x);
                        self.y.1 = self.y.1.max(y);
                        (x, y)
                    })
                    .collect(),
            );
        }

        self.y.0 = 0;
        self.y.1 += 1;
        let height = self.y.1 - self.y.0 + 1;
        self.x.0 -= height;
        self.x.1 += height;
        let width = self.x.1 - self.x.0 + 1;
        self.grid
            .extend((0..height).map(|_| vec![false; width as usize]));

        for coords in coords.iter() {
            for i in 0..coords.len() - 1 {
                let s = &coords[i];
                let e = &coords[i + 1];

                let sx = if s.0 < e.0 { s.0 } else { e.0 };
                let ex = if s.0 > e.0 { s.0 } else { e.0 };
                let sy = if s.1 < e.1 { s.1 } else { e.1 };
                let ey = if s.1 > e.1 { s.1 } else { e.1 };

                for y in sy..=ey {
                    for x in sx..=ex {
                        self.grid[(y - self.y.0) as usize][(x - self.x.0) as usize] = true;
                    }
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let width = self.x.1 - self.x.0 + 1;
        let height = self.y.1 - self.y.0 + 1;
        println!(
            "{:?} {:?} {} {} {}",
            self.x,
            self.y,
            width,
            height,
            (height as usize) * (width as usize)
        );
        Ok(self.solve::<false>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.solve::<true>().into())
    }
}
