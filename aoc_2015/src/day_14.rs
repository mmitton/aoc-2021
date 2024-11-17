#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug)]
struct Reindeer {
    _name: String,
    speed: usize,
    speed_sec: usize,
    cooldown: usize,
    distances: Vec<usize>,
    score: usize,
}

impl Reindeer {
    fn race(&mut self, total_time: usize) {
        let mut t = 0;
        let mut d = 0;
        while t < total_time {
            for _ in 0..self.speed_sec {
                self.distances.push(d);
                t += 1;
                d += self.speed;
            }
            for _ in 0..self.cooldown {
                self.distances.push(d);
                t += 1;
            }
        }
    }
}

impl FromStr for Reindeer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(" can fly ", " ");
        let s = s.replace(" km/s for ", " ");
        let s = s.replace(" seconds, but then must rest for ", " ");
        let s = s.replace(" seconds.", "");

        let parts: Vec<&str> = s.split(" ").collect();
        Ok(Self {
            _name: parts[0].to_string(),
            speed: parts[1].parse()?,
            speed_sec: parts[2].parse()?,
            cooldown: parts[3].parse()?,
            distances: Vec::new(),
            score: 0,
        })
    }
}

#[derive(Default)]
pub struct Day14 {
    reindeer: Vec<Reindeer>,
}

impl Day14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day14 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.reindeer.push(line.parse()?);
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day14 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        const TOTAL: usize = 2503;
        for reindeer in self.reindeer.iter_mut() {
            reindeer.race(TOTAL);
        }

        Ok(self
            .reindeer
            .iter()
            .map(|r| r.distances[TOTAL])
            .max()
            .unwrap()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        const TOTAL: usize = 2503;
        for reindeer in self.reindeer.iter_mut() {
            reindeer.race(TOTAL);
        }

        for t in 1..=TOTAL {
            let best_d = self.reindeer.iter().map(|r| r.distances[t]).max().unwrap();
            self.reindeer.iter_mut().for_each(|r| {
                if r.distances[t] == best_d {
                    r.score += 1
                }
            });
        }

        Ok(self.reindeer.iter().map(|r| r.score).max().unwrap().into())
    }
}
