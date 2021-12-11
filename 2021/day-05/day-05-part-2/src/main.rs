#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    NAN(std::num::ParseIntError),
}

struct Map {
    data: Vec<Vec<u16>>,
}

impl Map {
    fn print(&self) {
        for row in &self.data {
            for col in row {
                if *col == 0 {
                    print!(".");
                } else {
                    print!("{}", col);
                }
            }
            println!();
        }
    }

    fn mark_vent(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let x_max = if x1 > x2 { x1 } else { x2 };
        let y_max = if y1 > y2 { y1 } else { y2 };
        while self.data.len() <= y_max {
            let width = if self.data.len() == 0 {
                0
            } else {
                self.data[0].len()
            };
            self.data.push(vec![0; width]);
        }
        for row in &mut self.data {
            while row.len() <= x_max {
                row.push(0);
            }
        }

        println!("marking {},{} -> {},{}", x1, y1, x2, y2);
        if x1 == x2 {
            let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };

            for y in y1..=y2 {
                self.data[y][x1] += 1;
            }
        } else if y1 == y2 {
            let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };

            for x in x1..=x2 {
                self.data[y1][x] += 1;
            }
        } else {
            let x_delta = if x1 < x2 { 1 } else { -1 };
            let y_delta = if y1 < y2 { 1 } else { -1 };

            let mut y = y1;
            let mut x = x1;
            self.data[y][x] += 1;

            while y != y2 {
                y = (y as isize + y_delta) as usize;
                x = (x as isize + x_delta) as usize;
                self.data[y][x] += 1;
            }
        }
        if cfg!(debug_assertions) {
            self.print();
            println!();
        }
    }
}

fn load_map(filename: &str) -> Result<Map, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let mut map = Map { data: Vec::new() };
    let lines = BufReader::new(f).lines();

    fn get_coords(s: &str) -> Result<(usize, usize), Error> {
        let coords = s.trim().split(",").collect::<Vec<&str>>();

        if coords.len() != 2 {
            return Err(Error::InvalidInput(s.to_string()));
        }

        let x = coords[0].parse::<usize>().map_err(|e| Error::NAN(e))?;
        let y = coords[1].parse::<usize>().map_err(|e| Error::NAN(e))?;

        Ok((x, y))
    }

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let parts = line.split(" -> ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(Error::InvalidInput(line.to_string()));
        }

        let (x1, y1) = get_coords(parts[0])?;
        let (x2, y2) = get_coords(parts[1])?;

        map.mark_vent(x1, y1, x2, y2);
    }

    Ok(map)
}

fn main() -> Result<(), Error> {
    let map = load_map(INPUT_FILE)?;

    if cfg!(debug_assertions) {
        map.print();
    }

    let mut dangerous = 0usize;
    for row in &map.data {
        for col in row {
            if *col >= 2 {
                dangerous += 1;
            }
        }
    }

    println!("Answer: {}", dangerous);
    Ok(())
}
