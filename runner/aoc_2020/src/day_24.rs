#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::ops::Add;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Default)]
struct Tile {
    black: bool,
    neighbors: usize,
}

pub struct Day24 {
    initial: Vec<Coord>,
    tiles: HashMap<Coord, Tile>,
}

impl Day24 {
    pub fn new() -> Self {
        Self {
            initial: Vec::new(),
            tiles: HashMap::default(),
        }
    }

    fn flip(&mut self, c: Coord) {
        let tile = self.tiles.entry(c).or_default();
        let delta = if !tile.black { 1 } else { -1 } as usize;
        tile.black = !tile.black;

        for (dx, dy) in [(1, -1), (2, 0), (1, 1), (-1, 1), (-2, 0), (-1, -1)]
            .iter()
            .copied()
        {
            let neighbor = self.tiles.entry(c + (dx, dy).into()).or_default();
            neighbor.neighbors = neighbor.neighbors.wrapping_add(delta);
        }
    }
}

impl Runner for Day24 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for line in lines.iter() {
            let (mut x, mut y) = (0, 0);
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                match c {
                    'w' => x -= 2,
                    'e' => x += 2,
                    's' => {
                        y += 1;
                        match chars.next().unwrap() {
                            'e' => x += 1,
                            'w' => x -= 1,
                            _ => unreachable!(),
                        }
                    }
                    'n' => {
                        y -= 1;
                        match chars.next().unwrap() {
                            'e' => x += 1,
                            'w' => x -= 1,
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            self.flip((x, y).into());
            self.initial.push((x, y).into());
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.tiles.values().filter(|t| t.black).count().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut cur = HashMap::default();
        for _ in 0..100 {
            std::mem::swap(&mut cur, &mut self.tiles);
            self.tiles.clear();
            for (c, t) in cur.iter() {
                if t.black && (t.neighbors == 1 || t.neighbors == 2) {
                    self.flip(*c);
                } else if !t.black && t.neighbors == 2 {
                    self.flip(*c);
                }
            }
        }
        Ok(self.tiles.values().filter(|t| t.black).count().into())
    }
}
