#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, IterPairs, Lines, LinesOpt, Point2D};

#[allow(dead_code)]
#[derive(Debug)]
struct Region {
    c: char,
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    fn new(c: char, area: usize, perimeter: usize, sides: usize) -> Self {
        Self {
            c,
            area,
            perimeter,
            sides,
        }
    }

    fn cost_perimeter(&self) -> usize {
        self.area * self.perimeter
    }

    fn cost_sides(&self) -> usize {
        self.area * self.sides
    }
}

#[derive(Default)]
pub struct Day12 {
    map: Vec<Vec<char>>,
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }

    fn extract_regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut seen: HashSet<Point2D<usize>> = HashSet::default();

        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let p = Point2D::new(x, y);
                if seen.insert(p) {
                    const UP: usize = 0;
                    const DOWN: usize = 1;
                    const LEFT: usize = 2;
                    const RIGHT: usize = 3;
                    let mut i = 0;
                    let mut tiles = vec![p];
                    let mut perimeter = 0;
                    let mut sides = 0;
                    let mut side_points: [Vec<Point2D<usize>>; 4] =
                        std::array::from_fn(|_| Vec::new());
                    while i < tiles.len() {
                        for p2 in tiles[i].cardinal_neighbors() {
                            if let Some(c2) = self.map.get(p2.y).and_then(|r| r.get(p2.x)) {
                                if c2 == c {
                                    if seen.insert(p2) {
                                        tiles.push(p2);
                                    }
                                    continue;
                                }
                            }
                            if p2.x == tiles[i].x {
                                if p2.y == tiles[i].y + 1 {
                                    side_points[DOWN].push(tiles[i]);
                                } else {
                                    side_points[UP].push(tiles[i]);
                                }
                            } else if p2.x == tiles[i].x + 1 {
                                side_points[RIGHT].push(tiles[i]);
                            } else {
                                side_points[LEFT].push(tiles[i]);
                            }
                            perimeter += 1;
                        }
                        i += 1;
                    }

                    macro_rules! add_sides {
                        ($side_str: expr, $side:expr, $along:ident, $other:ident) => {{
                            sides += 1;
                            $side.sort_by_key(|p| (p.$along, p.$other));
                            for (a, b) in $side.iter().pairs() {
                                if a.$along != b.$along || a.$other + 1 != b.$other {
                                    sides += 1;
                                }
                            }
                        }};
                    }
                    add_sides!("UP", side_points[UP], y, x);
                    add_sides!("DOWN", side_points[DOWN], y, x);
                    add_sides!("LEFT", side_points[LEFT], x, y);
                    add_sides!("RIGHT", side_points[RIGHT], x, y);
                    regions.push(Region::new(*c, tiles.len(), perimeter, sides));
                }
            }
        }

        regions
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .extract_regions()
            .iter()
            .map(Region::cost_perimeter)
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .extract_regions()
            .iter()
            .map(Region::cost_sides)
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.map.push(line.chars().collect());
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
