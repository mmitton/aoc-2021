#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};

#[derive(Copy, Clone, Default, Debug)]
enum Dir {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, Default, Debug)]
struct Guard {
    pos: Point2D<u8>,
    dir: Dir,
}

impl Guard {
    fn next_pos(&self) -> Point2D<u8> {
        let (x, y) = match self.dir {
            Dir::Up => (self.pos.x, self.pos.y.wrapping_sub(1)),
            Dir::Right => (self.pos.x.wrapping_add(1), self.pos.y),
            Dir::Down => (self.pos.x, self.pos.y.wrapping_add(1)),
            Dir::Left => (self.pos.x.wrapping_sub(1), self.pos.y),
        };
        Point2D::new(x, y)
    }

    fn turn(&mut self) {
        match self.dir {
            Dir::Up => self.dir = Dir::Right,
            Dir::Right => self.dir = Dir::Down,
            Dir::Down => self.dir = Dir::Left,
            Dir::Left => self.dir = Dir::Up,
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
enum State {
    #[default]
    Empty,
    Visited,
    Obstruction,
}

#[derive(Clone, Default)]
struct Lab {
    guard: Guard,
    map: Vec<Vec<State>>,
    corners: HashSet<(Point2D<u8>, Point2D<u8>)>,
}

impl Lab {
    fn map_path(&mut self) -> bool {
        let mut next_pos = self.guard.next_pos();
        while let Some(s) = self
            .map
            .get_mut(next_pos.y as usize)
            .and_then(|r| r.get_mut(next_pos.x as usize))
        {
            match s {
                State::Empty | State::Visited => {
                    *s = State::Visited;
                    self.guard.pos = next_pos;
                }
                State::Obstruction => {
                    if !self.corners.insert((self.guard.pos, next_pos)) {
                        return true;
                    }
                    self.guard.turn();
                }
            }
            next_pos = self.guard.next_pos();
        }
        false
    }

    fn visited(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|s| matches!(s, State::Visited)).count())
            .sum::<usize>()
    }
}

#[derive(Default)]
pub struct Day06 {
    lab: Lab,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        self.lab.map_path();
        Ok(self.lab.visited().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut initial_lab = self.lab.clone();
        initial_lab.map_path();
        let mut ans = 0;
        for y in 0..self.lab.map.len() {
            for x in 0..self.lab.map[y].len() {
                if x == self.lab.guard.pos.x as usize && y == self.lab.guard.pos.y as usize {
                    continue;
                }
                if matches!(initial_lab.map[y][x], State::Visited) {
                    let mut lab = self.lab.clone();
                    lab.map[y][x] = State::Obstruction;
                    if lab.map_path() {
                        ans += 1;
                    }
                }
            }
        }
        Ok(ans.into())
    }
}

impl helper::Runner for Day06 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(match c {
                    '.' => State::Empty,
                    '#' => State::Obstruction,
                    '^' => {
                        self.lab.guard.pos.x = x as u8;
                        self.lab.guard.pos.y = y as u8;
                        self.lab.guard.dir = Dir::Up;
                        State::Visited
                    }
                    _ => unreachable!(),
                });
            }
            self.lab.map.push(row);
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
