#[allow(unused_imports)]
use helper::{print, println, Dijkstra, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};

#[derive(Default)]
pub struct Day16 {
    hall: HashSet<Point2D<usize>>,
    start: Point2D<usize>,
    end: Point2D<usize>,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn next(&self, p: Point2D<usize>) -> [(usize, (Point2D<usize>, Self)); 3] {
        match self {
            Self::North => [
                (1, (Point2D::new(p.x, p.y - 1), Self::North)),
                (1001, (Point2D::new(p.x + 1, p.y), Self::East)),
                (1001, (Point2D::new(p.x - 1, p.y), Self::West)),
            ],
            Self::South => [
                (1, (Point2D::new(p.x, p.y + 1), Self::South)),
                (1001, (Point2D::new(p.x + 1, p.y), Self::East)),
                (1001, (Point2D::new(p.x - 1, p.y), Self::West)),
            ],
            Self::East => [
                (1, (Point2D::new(p.x + 1, p.y), Self::East)),
                (1001, (Point2D::new(p.x, p.y - 1), Self::North)),
                (1001, (Point2D::new(p.x, p.y + 1), Self::South)),
            ],
            Self::West => [
                (1, (Point2D::new(p.x - 1, p.y), Self::West)),
                (1001, (Point2D::new(p.x, p.y - 1), Self::North)),
                (1001, (Point2D::new(p.x, p.y + 1), Self::South)),
            ],
        }
    }
}

impl Day16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn walk(&self) -> Result<(usize, usize), Error> {
        let (cost, paths) = Dijkstra::find_first_paths((self.start, Dir::East), |(at, dir)| {
            dir.next(at).into_iter().filter_map(|(cost, (at, dir))| {
                if self.hall.contains(&at) {
                    Some((cost, (at, dir), at == self.end))
                } else {
                    None
                }
            })
        })
        .ok_or(Error::Unsolved)?;

        let mut points = HashSet::default();
        for path in paths.iter() {
            for (point, _) in path.iter() {
                points.insert(point);
            }
        }
        Ok((cost, points.len()))
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.walk()?.0.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.walk()?.1.into())
    }
}

impl helper::Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let p = Point2D::new(x, y);
                match c {
                    'S' => {
                        self.start = p;
                        self.hall.insert(p)
                    }
                    'E' => {
                        self.end = p;
                        self.hall.insert(p)
                    }
                    '.' => self.hall.insert(p),
                    '#' => false,
                    _ => return Err(Error::InvalidInput(row.into())),
                };
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
