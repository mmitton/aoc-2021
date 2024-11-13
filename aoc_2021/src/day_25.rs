#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug, Copy, Clone)]
enum State {
    Empty,
    East,
    South,
}

#[derive(Default, Debug)]
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
        #[allow(clippy::needless_range_loop)]
        for y in 0..height {
            for x in 0..width {
                let nx = (x + 1) % width;
                if let (State::East, State::Empty) = (cur[y][x], cur[y][nx]) {
                    self.data[y][x] = State::Empty;
                    self.data[y][nx] = State::East;
                    moved = true;
                }
            }
        }

        // Then South
        let cur = self.data.clone();
        for y in 0..height {
            for x in 0..width {
                let ny = (y + 1) % height;
                if let (State::South, State::Empty) = (cur[y][x], cur[ny][x]) {
                    self.data[y][x] = State::Empty;
                    self.data[ny][x] = State::South;
                    moved = true;
                }
            }
        }

        moved
    }

    fn _print(&self) {
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

pub struct Day25 {
    map: Map,
}

impl Day25 {
    pub fn new() -> Self {
        Self {
            map: Map::default(),
        }
    }
}

impl Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::TRIM)?;
        for line in lines.iter() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '.' => State::Empty,
                    '>' => State::East,
                    'v' => State::South,
                    _ => return Err(Error::InvalidInput(line.to_string())),
                });
            }

            self.map.data.push(row);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        for i in 1.. {
            if !self.map.tick() {
                return Ok(i.into());
            }
        }
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Skipped)
    }
}
