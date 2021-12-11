#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidInput(String),
}

struct Map(Vec<Vec<u8>>);

impl Map {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn height_at(&self, x: usize, y: usize) -> u8 {
        self.0[y][x]
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let cur = self.height_at(x, y);
        if x > 0 && self.height_at(x - 1, y) <= cur {
            return false;
        }
        if x < self.width() - 1 && self.height_at(x + 1, y) <= cur {
            return false;
        }
        if y > 0 && self.height_at(x, y - 1) <= cur {
            return false;
        }
        if y < self.height() - 1 && self.height_at(x, y + 1) <= cur {
            return false;
        }

        true
    }

    fn print(&self) {
        println!("Map:  {}x{}", self.width(), self.height());
        for row in &self.0 {
            print!("  ");
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }
}

fn load_map(filename: &str) -> Result<Map, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut map = Map(Vec::new());

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if map.0.len() != 0 && map.0[0].len() != line.len() {
            return Err(Error::InvalidInput(line.to_string()));
        }

        let mut row = Vec::new();
        for i in 0..line.len() {
            let c = line[i..i + 1].parse::<u8>().map_err(|e| Error::NAN(e))?;
            row.push(c);
        }

        map.0.push(row);
    }

    Ok(map)
}

fn main() -> Result<(), Error> {
    let map = load_map(INPUT_FILE)?;

    map.print();

    let mut risk = 0usize;
    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.is_low_point(x, y) {
                let height = map.height_at(x, y);
                risk += height as usize + 1;
                println!("Found low point at {},{}  {}", x, y, height);
            }
        }
    }

    println!("Risk: {}", risk);

    Ok(())
}
