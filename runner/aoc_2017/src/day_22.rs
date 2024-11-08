#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

enum State {
    Weakened,
    Flagged,
    Infected,
}

#[derive(Default)]
struct Carrier {
    x: i16,
    y: i16,
    dir: u8,
}

#[derive(Default)]
struct Map {
    nodes: HashMap<(i16, i16), State>,
}

impl Carrier {
    fn burst_1(&mut self, map: &mut Map) -> bool {
        let infected = if let Some(State::Infected) = map.nodes.get(&(self.x, self.y)) {
            if self.dir == 3 {
                self.dir = 0;
            } else {
                self.dir += 1;
            }
            map.nodes.remove(&(self.x, self.y));
            false
        } else {
            if self.dir == 0 {
                self.dir = 3;
            } else {
                self.dir -= 1;
            }
            map.nodes.insert((self.x, self.y), State::Infected);
            true
        };

        match self.dir {
            0 => self.y -= 1,
            1 => self.x += 1,
            2 => self.y += 1,
            3 => self.x -= 1,
            _ => unreachable!(),
        }

        infected
    }

    fn burst_2(&mut self, map: &mut Map) -> bool {
        let mut set_infected = false;
        let state = map.nodes.get(&(self.x, self.y));
        match state {
            None => {
                if self.dir == 0 {
                    self.dir = 3;
                } else {
                    self.dir -= 1;
                }
                map.nodes.insert((self.x, self.y), State::Weakened);
            }
            Some(State::Weakened) => {
                map.nodes.insert((self.x, self.y), State::Infected);
                set_infected = true;
            }
            Some(State::Infected) => {
                if self.dir == 3 {
                    self.dir = 0;
                } else {
                    self.dir += 1;
                }
                map.nodes.insert((self.x, self.y), State::Flagged);
            }
            Some(State::Flagged) => {
                match self.dir {
                    0 => self.dir = 2,
                    1 => self.dir = 3,
                    2 => self.dir = 0,
                    3 => self.dir = 1,
                    _ => unreachable!(),
                }
                map.nodes.remove(&(self.x, self.y));
            }
        }

        match self.dir {
            0 => self.y -= 1,
            1 => self.x += 1,
            2 => self.y += 1,
            3 => self.x -= 1,
            _ => unreachable!(),
        }

        set_infected
    }
}

#[derive(Default)]
pub struct Day22 {
    carrier: Carrier,
    map: Map,
}

impl Day22 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day22 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.carrier.x = lines[0].len() as i16 / 2;
        self.carrier.y = lines.len() as i16 / 2;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    self.map.nodes.insert((x as i16, y as i16), State::Infected);
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut infected_bursts = 0;
        for _ in 0..10000 {
            if self.carrier.burst_1(&mut self.map) {
                infected_bursts += 1;
            }
        }
        Ok(infected_bursts.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut infected_bursts = 0;
        for _ in 0..10000000 {
            if self.carrier.burst_2(&mut self.map) {
                infected_bursts += 1;
            }
        }
        Ok(infected_bursts.into())
    }
}
