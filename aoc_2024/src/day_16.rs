#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};
use std::collections::BTreeMap;

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
        let mut work: BTreeMap<usize, HashSet<(Point2D<usize>, Dir)>> = BTreeMap::new();
        work.entry(0).or_default().insert((self.start, Dir::East));

        #[derive(Debug)]
        struct SeenEntry {
            cost: usize,
            from: HashSet<(Point2D<usize>, Dir)>,
        }

        let mut seen: HashMap<(Point2D<usize>, Dir), SeenEntry> = HashMap::default();
        seen.insert(
            (self.start, Dir::East),
            SeenEntry {
                cost: 0,
                from: HashSet::default(),
            },
        );

        let mut walked_along = HashSet::default();
        while let Some((cost, mut cost_work)) = work.pop_first() {
            for (cur, dir) in cost_work.drain() {
                if cur == self.end {
                    let mut work = vec![(self.end, dir)];
                    walked_along.insert(self.end);

                    while let Some((at, dir)) = work.pop() {
                        if let Some(seen_entry) = seen.get(&(at, dir)) {
                            for (from, from_dir) in seen_entry.from.iter() {
                                walked_along.insert(*from);
                                work.push((*from, *from_dir));
                            }
                        }
                    }
                    continue;
                }
                if let Some(seen_cost) = seen.get(&(cur, dir)) {
                    if seen_cost.cost < cost {
                        continue;
                    }
                }
                for (next_cost, (next, next_dir)) in dir.next(cur) {
                    if !self.hall.contains(&next) {
                        continue;
                    }

                    let next_cost = cost + next_cost;
                    let seen_cost = seen.entry((next, next_dir)).or_insert(SeenEntry {
                        cost: usize::MAX,
                        from: HashSet::default(),
                    });
                    use std::cmp::Ordering;
                    match seen_cost.cost.cmp(&next_cost) {
                        Ordering::Less => continue,
                        Ordering::Equal => {
                            seen_cost.from.insert((cur, dir));
                            continue;
                        }
                        Ordering::Greater => {
                            seen_cost.cost = next_cost;
                            seen_cost.from.clear();
                            seen_cost.from.insert((cur, dir));
                        }
                    }
                    work.entry(next_cost).or_default().insert((next, next_dir));
                }
            }

            if !walked_along.is_empty() {
                return Ok((cost, walked_along.len()));
            }
        }

        Err(Error::Unsolved)
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
