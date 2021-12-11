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

type Point = (usize, usize);
struct Basin {
    points: Vec<Point>,
}

struct Map {
    depths: Vec<Vec<u8>>,
    basins: Vec<Basin>,
}

impl Map {
    fn width(&self) -> usize {
        self.depths[0].len()
    }

    fn height(&self) -> usize {
        self.depths.len()
    }

    fn find_basins(&mut self) {
        for y in 0..self.height() {
            'x_loop: for x in 0..self.width() {
                let cur = self.height_at(x, y);
                if cur == 9 {
                    continue 'x_loop;
                }
                for basin in &self.basins {
                    if basin.points.contains(&(x, y)) {
                        continue 'x_loop;
                    }
                }

                fn expand(map: &Map, points: &mut Vec<Point>, x: usize, y: usize) {
                    let cur = map.height_at(x, y);
                    if cur == 9 {
                        return;
                    }

                    if points.contains(&(x, y)) {
                        return;
                    }
                    points.push((x, y));

                    if x != 0 {
                        expand(map, points, x - 1, y);
                    }
                    if x != map.width() - 1 {
                        expand(map, points, x + 1, y);
                    }
                    if y != 0 {
                        expand(map, points, x, y - 1);
                    }
                    if y != map.height() - 1 {
                        expand(map, points, x, y + 1);
                    }
                }

                let mut points: Vec<Point> = Vec::new();
                expand(self, &mut points, x, y);

                println!("Found Basin: {:?}", points);

                self.basins.push(Basin { points: points });
            }
        }
    }

    fn height_at(&self, x: usize, y: usize) -> u8 {
        self.depths[y][x]
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
        for row in &self.depths {
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

    let mut map = Map {
        depths: Vec::new(),
        basins: Vec::new(),
    };

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if map.depths.len() != 0 && map.depths[0].len() != line.len() {
            return Err(Error::InvalidInput(line.to_string()));
        }

        let mut row = Vec::new();
        for i in 0..line.len() {
            let c = line[i..i + 1].parse::<u8>().map_err(|e| Error::NAN(e))?;
            row.push(c);
        }

        map.depths.push(row);
    }

    Ok(map)
}

fn main() -> Result<(), Error> {
    let mut map = load_map(INPUT_FILE)?;

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

    map.find_basins();

    let mut basin_sizes = Vec::new();
    for basin in &map.basins {
        basin_sizes.push(basin.points.len());
    }
    basin_sizes.sort();
    basin_sizes.reverse();

    println!("3 biggest basin sizes: {:?}", &basin_sizes[..3]);
    println!(
        "Answer: {}",
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    );

    Ok(())
}
