#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(Error::InvalidInput(format!("Invalid move {value:?}"))),
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Tile {
    Box,
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(Default)]
pub struct Day15 {
    tiles: HashMap<Point2D<usize>, Tile>,
    moves: Vec<Move>,
    robot: Point2D<usize>,
}

impl Day15 {
    pub fn new() -> Self {
        Self::default()
    }

    fn _print(&self) {
        let height = self.tiles.keys().map(|p| p.y).max().unwrap();
        let width = self.tiles.keys().map(|p| p.x).max().unwrap();

        for y in 0..=height {
            for x in 0..=width {
                let p = Point2D::new(x, y);
                let c = match self.tiles.get(&p) {
                    Some(Tile::Box) => 'O',
                    Some(Tile::BoxLeft) => '[',
                    Some(Tile::BoxRight) => ']',
                    Some(Tile::Wall) => '#',
                    None => {
                        if p == self.robot {
                            '@'
                        } else {
                            '.'
                        }
                    }
                };
                print!("{c}");
            }
            println!();
        }
    }

    fn move_robot(&mut self) {
        'moves: for m in self.moves.iter() {
            let delta = match m {
                Move::Up => Point2D::new(0, usize::MAX),
                Move::Down => Point2D::new(0, 1),
                Move::Left => Point2D::new(usize::MAX, 0),
                Move::Right => Point2D::new(1, 0),
            };
            let mut boxes_to_move = HashSet::default();
            let mut cur = vec![self.robot];
            loop {
                let mut next = Vec::new();
                for p in cur.iter() {
                    let next_p = *p + delta;
                    match self.tiles.get(&next_p) {
                        None => {}
                        Some(Tile::Wall) => continue 'moves,
                        Some(Tile::Box) => {
                            if boxes_to_move.insert((next_p, Tile::Box)) {
                                next.push(next_p);
                            }
                        }
                        Some(Tile::BoxLeft) => {
                            if boxes_to_move.insert((next_p, Tile::BoxLeft)) {
                                next.push(next_p);
                            }
                            let next_p = next_p + Point2D::new(1, 0);
                            if boxes_to_move.insert((next_p, Tile::BoxRight)) {
                                next.push(next_p);
                            }
                        }
                        Some(Tile::BoxRight) => {
                            if boxes_to_move.insert((next_p, Tile::BoxRight)) {
                                next.push(next_p);
                            }
                            let next_p = next_p - Point2D::new(1, 0);
                            if boxes_to_move.insert((next_p, Tile::BoxLeft)) {
                                next.push(next_p);
                            }
                        }
                    }
                }
                if next.is_empty() {
                    break;
                }
                cur = next;
            }

            // Remove first
            for (p, _) in boxes_to_move.iter() {
                self.tiles.remove(p);
            }
            // Add back in, but moved
            for (p, t) in boxes_to_move.iter() {
                self.tiles.insert(*p + delta, *t);
            }
            self.robot += delta;
        }
    }

    fn gps_sum(&self) -> usize {
        self.tiles
            .iter()
            .filter_map(|(p, t)| match t {
                Tile::Box | Tile::BoxLeft => Some(p.y * 100 + p.x),
                Tile::Wall | Tile::BoxRight => None,
            })
            .sum()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        self.move_robot();
        Ok(self.gps_sum().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut new_tiles = HashMap::default();
        for (p, t) in self.tiles.iter() {
            let new_p = *p * Point2D::new(2, 1);
            match t {
                Tile::Wall => {
                    new_tiles.insert(new_p, Tile::Wall);
                    new_tiles.insert(new_p + Point2D::new(1, 0), Tile::Wall);
                }
                Tile::Box => {
                    new_tiles.insert(new_p, Tile::BoxLeft);
                    new_tiles.insert(new_p + Point2D::new(1, 0), Tile::BoxRight);
                }
                _ => unreachable!(),
            }
        }
        self.tiles = new_tiles;
        self.robot *= Point2D::new(2, 1);
        self.move_robot();
        Ok(self.gps_sum().into())
    }
}

impl helper::Runner for Day15 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();
        for (y, line) in lines.by_ref().enumerate() {
            if line.is_empty() {
                break;
            }
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        _ = self.tiles.insert(Point2D::new(x, y), Tile::Wall);
                    }
                    'O' => {
                        _ = self.tiles.insert(Point2D::new(x, y), Tile::Box);
                    }
                    '@' => self.robot = Point2D::new(x, y),
                    '.' => {}
                    _ => return Err(Error::InvalidInput(format!("Invalid map tile {c:?}"))),
                }
            }
        }

        for line in lines {
            for c in line.chars() {
                self.moves.push(c.try_into()?);
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
