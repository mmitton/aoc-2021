#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

struct Vein {
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
}

impl FromStr for Vein {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidInput(s.to_string()));
        }

        let mut x0 = 0;
        let mut x1 = 0;
        let mut y0 = 0;
        let mut y1 = 0;

        fn parse(s: &str) -> Result<(char, usize, usize), Error> {
            let axis = s.chars().next().unwrap();
            let parts: Vec<&str> = s.split("=").collect();
            if parts.len() != 2 {
                return Err(Error::InvalidInput(s.to_string()));
            }

            let (min, max) = if parts[1].contains("..") {
                let parts: Vec<&str> = parts[1].split("..").collect();
                (parts[0].parse()?, parts[1].parse()?)
            } else {
                let v: usize = parts[1].parse()?;
                (v, v)
            };

            Ok((axis, min, max))
        }

        for part in &parts {
            let split = parse(part)?;
            if split.0 == 'x' {
                x0 = split.1;
                x1 = split.2;
            } else {
                y0 = split.1;
                y1 = split.2;
            }
        }

        Ok(Vein { x0, x1, y0, y1 })
    }
}

struct Map {
    veins: Vec<Vein>,
    spring: (usize, usize),
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
    tiles: Vec<Vec<char>>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            veins: Vec::new(),
            spring: (500, 0),
            x0: 500,
            x1: 500,
            y0: usize::MAX,
            y1: usize::MIN,
            tiles: Vec::new(),
        }
    }
}

impl Map {
    fn get(&self, x: usize, y: usize) -> char {
        self.tiles[y][x]
    }

    fn set(&mut self, x: usize, y: usize, c: char) -> bool {
        let old = self.tiles[y][x];
        self.tiles[y][x] = c;
        old != c
    }

    fn add_vein(&mut self, v: Vein) {
        if v.x0 - 2 < self.x0 {
            self.x0 = v.x0 - 2;
        }
        if v.x1 + 2 > self.x1 {
            self.x1 = v.x1 + 2;
        }
        if v.y0 < self.y0 {
            self.y0 = v.y0;
        }
        if v.y1 > self.y1 {
            self.y1 = v.y1;
        }
        self.veins.push(v);
    }

    fn build_tiles(&mut self) {
        for _ in 0..=self.y1 {
            self.tiles.push(vec!['.'; self.x1 + 1]);
        }

        self.tiles[self.spring.1][self.spring.0] = '+';
        self.tiles[self.spring.1 + 1][self.spring.0] = '|';

        for v in &self.veins {
            for y in v.y0..=v.y1 {
                for x in v.x0..=v.x1 {
                    self.tiles[y][x] = '#';
                }
            }
        }
    }

    fn water_tiles(&self) -> usize {
        let mut tiles = 0;
        for y in self.y0..=self.y1 {
            for x in self.x0..=self.x1 {
                if self.get(x, y) == '~' {
                    tiles += 1;
                }
            }
        }
        tiles
    }

    fn fall(&mut self) -> bool {
        let mut done = true;
        let mut neighbors = Vec::new();
        for y in 0..self.y1 {
            for x in self.x0..=self.x1 {
                let c = self.get(x, y);
                if c == '|' {
                    match self.get(x, y + 1) {
                        '.' => {
                            // Fall
                            self.set(x, y + 1, '|');
                            done = false;
                        }
                        '#' | '~' => {
                            // Fill to the sides
                            neighbors.clear();
                            let mut spills = false;

                            let mut nx = x - 1;
                            loop {
                                if self.get(nx, y) == '#' {
                                    break;
                                }
                                neighbors.push((nx, y));
                                match self.get(nx, y + 1) {
                                    '|' | '.' => {
                                        spills = true;
                                        break;
                                    }
                                    _ => {}
                                }
                                nx -= 1;
                            }

                            nx = x + 1;
                            loop {
                                if self.get(nx, y) == '#' {
                                    break;
                                }
                                neighbors.push((nx, y));
                                match self.get(nx, y + 1) {
                                    '|' | '.' => {
                                        spills = true;
                                        break;
                                    }
                                    _ => {}
                                }
                                nx += 1;
                            }

                            let c = if spills { '|' } else { '~' };
                            for cell in &neighbors {
                                if self.set(cell.0, cell.1, c) {
                                    done = false;
                                }
                            }
                            if self.set(x, y, c) {
                                done = false;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        done
    }
}

#[derive(Default)]
pub struct Day17 {
    map: Map,
}

impl Day17 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day17 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.map.add_vein(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.map.build_tiles();
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.map.build_tiles();
        Ok(self.map.water_tiles().into())
    }
}
