use std::collections::VecDeque;

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Map(Vec<Vec<u8>>);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Clone)]
struct Path {
    risk: usize,
    points: Vec<Point>,
}

impl Path {
    fn add(&mut self, p: Point, risk: u8) {
        self.points.push(p);
        self.risk += risk as usize;
    }
}

impl Map {
    fn print(&self) {
        println!("Map");
        for row in &self.0 {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }

    fn find_shortest_path(&self) -> Path {
        let mut initial_path = Path::default();
        initial_path.add(Point { x: 0, y: 0 }, 0);
        let mut paths: Vec<Path> = Vec::new();
        paths.push(initial_path);

        let mut lowest_risk = Vec::new();
        for _ in 0..self.0.len() {
            lowest_risk.push(vec![!0usize; self.0[0].len()]);
        }

        let max_x = self.0[0].len() - 1;
        let max_y = self.0.len() - 1;
        lowest_risk[0][0] = 0;

        println!("max_x:{}  max_y:{}", max_x, max_y);

        let mut completed_paths = Vec::new();
        let mut best_completed = !0usize;

        while paths.len() > 0 {
            paths.sort_by_key(|p| -(p.risk as isize));
            let path = paths.pop().unwrap();

            if path.risk > best_completed {
                continue;
            }

            let x = path.points[path.points.len() - 1].x;
            let y = path.points[path.points.len() - 1].y;

            if x != max_x || y != max_y {
                for m in &[(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
                    if (x == 0 && m.0 == -1)
                        || (y == 0 && m.1 == -1)
                        || (x == self.0[0].len() - 1 && m.0 == 1)
                        || (y == self.0.len() - 1 && m.1 == 1)
                    {
                        continue;
                    }

                    let new_x = (x as isize + m.0) as usize;
                    let new_y = (y as isize + m.1) as usize;
                    let p = Point { x: new_x, y: new_y };

                    let cell_risk = self.0[new_y][new_x];
                    let new_risk = path.risk + cell_risk as usize;

                    if lowest_risk[new_y][new_x] <= new_risk {
                        continue;
                    }
                    lowest_risk[new_y][new_x] = new_risk;

                    let mut new_path = path.clone();
                    new_path.add(p, cell_risk);
                    if new_x == max_x && new_y == max_y {
                        if new_path.risk < best_completed {
                            best_completed = new_path.risk;
                        }
                        paths.retain(|p| p.risk < best_completed);
                        completed_paths.push(new_path);
                    } else {
                        paths.push(new_path);
                    }
                }
            }
        }

        completed_paths.sort_by_key(|p| p.risk);
        completed_paths.remove(0)
    }
}

fn load_input(filename: &str) -> Result<Map, Error> {
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

        let mut row = Vec::new();
        for c in 0..line.len() {
            let n: u8 = line[c..c + 1].parse().map_err(|e| Error::NAN(e))?;
            row.push(n);
        }

        map.0.push(row);
    }

    Ok(map)
}

fn main() -> Result<(), Error> {
    let map = load_input(INPUT_FILE)?;

    if cfg!(debug_assertions) {
        map.print();
    }

    let shortest_path = map.find_shortest_path();

    println!("shortest_path: {:?}", shortest_path);

    Ok(())
}
