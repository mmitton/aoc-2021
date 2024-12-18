use helper::Dijkstra;
#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};

#[derive(Default)]
pub struct Day18 {
    points: Vec<Point2D<usize>>,
}

impl Day18 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut map = HashSet::default();
        let (t, exit) = if self.points.len() == 25 {
            (12, Point2D::<usize>::new(6, 6))
        } else {
            (1024, Point2D::<usize>::new(70, 70))
        };

        for p in self.points.iter().take(t) {
            map.insert(*p);
        }

        Ok(Dijkstra::find_first(Point2D::<usize>::new(0, 0), |at| {
            at.cardinal_neighbors().into_iter().filter_map(|n| {
                if !map.contains(&n) && n.x <= exit.x && n.y <= exit.y {
                    Some((1, n, n == exit))
                } else {
                    None
                }
            })
        })
        .unwrap()
        .0
        .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let exit = if self.points.len() == 25 {
            Point2D::<usize>::new(6, 6)
        } else {
            Point2D::<usize>::new(70, 70)
        };

        // Do a binary search for the first drop that causes no path to be found
        let mut map = HashSet::default();
        let mut width = self.points.len() / 2;
        let mut at = width;
        let mut no_path_at = usize::MAX;
        loop {
            if at == no_path_at {
                return Ok(format!("{}", self.points[at]).into());
            }

            if at + 1 > map.len() {
                for p in self.points.iter().take(at + 1).skip(map.len()) {
                    map.insert(*p);
                }
            } else {
                for p in self.points.iter().take(map.len()).skip(at + 1) {
                    map.remove(p);
                }
            }

            width = (width + 1) / 2;
            match Dijkstra::find_first(Point2D::<usize>::new(0, 0), |at| {
                at.cardinal_neighbors().into_iter().filter_map(|n| {
                    if !map.contains(&n) && n.x <= exit.x && n.y <= exit.y {
                        Some((1, n, n == exit))
                    } else {
                        None
                    }
                })
            }) {
                Some(_) => {
                    at += width;
                }
                None => {
                    no_path_at = at;
                    at -= width;
                }
            }
        }
    }
}

impl helper::Runner for Day18 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let Some((x, y)) = line.split_once(',') else {
                return Err(Error::InvalidInput(line.into()));
            };
            self.points.push(Point2D::new(x.parse()?, y.parse()?));
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
