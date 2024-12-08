#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};

#[derive(Copy, Clone, Default, Debug)]
enum Dir {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, Default, Debug)]
struct Guard {
    pos: Point2D<u8>,
    dir: Dir,
}

#[derive(Default)]
pub struct Day06 {
    guard: Guard,
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }

    fn walk_path<F>(&self, mut walked: F) -> bool
    where
        F: FnMut(Point2D<u8>, Point2D<u8>),
    {
        let mut guard = self.guard;
        let mut seen = HashSet::default();
        loop {
            let (next_pos, dir, done) = match guard.dir {
                Dir::Up => self.cols[guard.pos.x as usize]
                    .iter()
                    .copied()
                    .map(|y| (Point2D::new(guard.pos.x, y), Dir::Right))
                    .rev()
                    .find(|(next_pos, _)| next_pos.y < guard.pos.y)
                    .map(|(mut next_pos, dir)| {
                        next_pos.y += 1;
                        (next_pos, dir, false)
                    })
                    .unwrap_or((Point2D::new(guard.pos.x, 0), Dir::Up, true)),
                Dir::Down => self.cols[guard.pos.x as usize]
                    .iter()
                    .copied()
                    .map(|y| (Point2D::new(guard.pos.x, y), Dir::Left))
                    .find(|(next_pos, _)| next_pos.y > guard.pos.y)
                    .map(|(mut next_pos, dir)| {
                        next_pos.y -= 1;
                        (next_pos, dir, false)
                    })
                    .unwrap_or((
                        Point2D::new(guard.pos.x, self.rows.len() as u8 - 1),
                        Dir::Down,
                        true,
                    )),
                Dir::Left => self.rows[guard.pos.y as usize]
                    .iter()
                    .copied()
                    .map(|x| (Point2D::new(x, guard.pos.y), Dir::Up))
                    .rev()
                    .find(|(next_pos, _)| next_pos.x < guard.pos.x)
                    .map(|(mut next_pos, dir)| {
                        next_pos.x += 1;
                        (next_pos, dir, false)
                    })
                    .unwrap_or((Point2D::new(0, guard.pos.y), Dir::Left, true)),
                Dir::Right => self.rows[guard.pos.y as usize]
                    .iter()
                    .copied()
                    .map(|x| (Point2D::new(x, guard.pos.y), Dir::Down))
                    .find(|(next_pos, _)| next_pos.x > guard.pos.x)
                    .map(|(mut next_pos, dir)| {
                        next_pos.x -= 1;
                        (next_pos, dir, false)
                    })
                    .unwrap_or((
                        Point2D::new(self.cols.len() as u8 - 1, guard.pos.y),
                        Dir::Left,
                        true,
                    )),
            };
            if !seen.insert((guard.pos, next_pos)) {
                return true;
            }
            walked(guard.pos, next_pos);
            guard.pos = next_pos;
            guard.dir = dir;
            if done {
                return false;
            }
        }
    }

    fn initial_walked(&self) -> HashSet<Point2D<u8>> {
        let mut walked = HashSet::default();
        assert!(!self.walk_path(|from, to| {
            if from.x != to.x && from.y == to.y {
                let min_x = from.x.min(to.x);
                let max_x = from.x.max(to.x);
                for x in min_x..=max_x {
                    walked.insert(Point2D::new(x, from.y));
                }
            } else if from.y != to.y && from.x == to.x {
                let min_y = from.y.min(to.y);
                let max_y = from.y.max(to.y);
                for y in min_y..=max_y {
                    walked.insert(Point2D::new(from.x, y));
                }
            } else {
                unreachable!()
            }
        }));
        walked
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let walked = self.initial_walked();
        Ok(walked.len().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .initial_walked()
            .iter()
            .copied()
            .filter(|try_at| {
                if *try_at == self.guard.pos {
                    false
                } else {
                    self.rows[try_at.y as usize].push(try_at.x);
                    self.cols[try_at.x as usize].push(try_at.y);
                    self.rows[try_at.y as usize].sort();
                    self.cols[try_at.x as usize].sort();

                    let is_loop = self.walk_path(|_, _| {});

                    self.rows[try_at.y as usize].retain(|x| *x != try_at.x);
                    self.cols[try_at.x as usize].retain(|y| *y != try_at.y);

                    is_loop
                }
            })
            .count()
            .into())
    }
}

impl helper::Runner for Day06 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.rows.resize(lines.len(), Vec::new());
        self.cols.resize(lines[0].len(), Vec::new());
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => {
                        self.rows[y].push(x as u8);
                        self.cols[x].push(y as u8);
                    }
                    '^' => {
                        self.guard.pos.x = x as u8;
                        self.guard.pos.y = y as u8;
                        self.guard.dir = Dir::Up;
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
