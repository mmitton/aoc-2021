#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
}

#[derive(Debug, Copy, Clone)]
enum State {
    Empty,
    East,
    South,
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<State>>,
}

impl Map {
    fn tick(&mut self) -> bool {
        let mut moved = false;

        let height = self.data.len();
        let width = self.data[0].len();

        // East first
        let cur = self.data.clone();
        for y in 0..height {
            for x in 0..width {
                let nx = (x + 1) % width;
                match (cur[y][x], cur[y][nx]) {
                    (State::East, State::Empty) => {
                        self.data[y][x] = State::Empty;
                        self.data[y][nx] = State::East;
                        moved = true;
                    }
                    _ => {}
                }
            }
        }

        // Then South
        let cur = self.data.clone();
        for y in 0..height {
            for x in 0..width {
                let ny = (y + 1) % height;
                match (cur[y][x], cur[ny][x]) {
                    (State::South, State::Empty) => {
                        self.data[y][x] = State::Empty;
                        self.data[ny][x] = State::South;
                        moved = true;
                    }
                    _ => {}
                }
            }
        }

        moved
    }

    fn print(&self) {
        for row in &self.data {
            for cell in row {
                let cell = match cell {
                    State::Empty => ".",
                    State::East => ">",
                    State::South => "v",
                };
                print!("{}", cell);
            }
            println!();
        }
    }
}

fn load_input(filename: &str) -> Result<Map, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut map = Map { data: Vec::new() };

    for line in lines {
        let line = line.unwrap();
        let line = line.trim();

        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '.' => State::Empty,
                '>' => State::East,
                'v' => State::South,
                _ => return Err(Error::InvalidInput(line.to_string())),
            });
        }

        map.data.push(row);
    }

    Ok(map)
}

fn main() -> Result<(), Error> {
    let mut map = load_input(INPUT_FILE)?;

    for i in 1..usize::MAX {
        if !map.tick() {
            println!("Stopped after {} moves", i);
            break;
        }

        if cfg!(debug_assertions) {
            println!("Step {}", i);
            map.print();
        }
    }

    Ok(())
}
