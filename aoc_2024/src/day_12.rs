use helper::Point2D;
#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

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
                            let mut min = usize::MAX;
                            let mut max = usize::MIN;
                            for p in $side.iter() {
                                min = min.min(p.$along);
                                max = max.max(p.$along);
                            }
                            for along in min..=max {
                                let mut points: Vec<&Point2D<usize>> =
                                    $side.iter().filter(|p| p.$along == along).collect();
                                if !points.is_empty() {
                                    points.sort();
                                    sides += 1;
                                    for win in points.windows(2) {
                                        if win[0].$other + 1 != win[1].$other {
                                            sides += 1;
                                        }
                                    }
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
