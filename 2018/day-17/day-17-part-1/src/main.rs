#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidVein(String),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

struct Vein {
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
}

impl TryFrom<&str> for Vein {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(", ").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidVein(s.to_string()));
        }

        let mut x0 = 0;
        let mut x1 = 0;
        let mut y0 = 0;
        let mut y1 = 0;

        fn parse(s: &str) -> Result<(char, usize, usize), Error> {
            let axis = s.chars().nth(0).unwrap();
            let parts: Vec<&str> = s.split("=").collect();
            if parts.len() != 2 {
                return Err(Error::InvalidVein(s.to_string()));
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

        Ok(Vein {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
        })
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

impl Map {
    fn new() -> Self {
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

    fn get(&self, x: usize, y: usize) -> char {
        return self.tiles[y][x];
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
                match self.get(x, y) {
                    '|' | '~' => tiles += 1,
                    _ => {}
                }
            }
        }
        tiles
    }

    fn print(&self) {
        for y in 0..=self.y1 {
            for x in self.x0..=self.x1 {
                let c = self.get(x, y);
                print!("{}", c);
            }
            println!();
        }
        println!();
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

fn load_input(filename: &str) -> Result<Map, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut map = Map::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        map.add_vein(line.try_into()?);
    }

    map.build_tiles();

    Ok(map)
}

fn main() -> Result<(), Error> {
    let mut map = load_input(INPUT_FILE)?;

    if cfg!(debug_assertions) {
        map.print();
    }

    while !map.fall() {
        if cfg!(debug_assertions) {
            map.print();
        }
    }
    if cfg!(debug_assertions) {
        map.print();
    }

    println!("Water reaches: {}", map.water_tiles());

    Ok(())
}
