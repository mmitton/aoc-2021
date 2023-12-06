#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn is_win(&self, hold: usize) -> bool {
        (self.time - hold) * hold > self.distance
    }

    fn wins(&self) -> RangeInclusive<usize> {
        // distance = (time - hold) * hold
        // distance = time * hold - hold*hold
        // hold*hold - time*hold + distance = 0
        // a = 1
        // b = -time
        // c = distance
        let a = 1_f64;
        let b = -(self.time as f64);
        let c = self.distance as f64;

        let mut roots = [
            (-b + ((b * b) - 4. * a * c).sqrt()) / (2. * a),
            (-b - ((b * b) - 4. * a * c).sqrt()) / (2. * a),
        ];

        if roots[0] > roots[1] {
            roots.swap(0, 1);
        }

        let mut min = roots[0].ceil() as usize;
        let mut max = roots[1].floor() as usize;

        // Might need to up min or down max as we cannot be exactly distance, must exceed it.
        // Quadratic only gives roots for exactly going distance
        if !self.is_win(min) {
            min += 1;
        }
        if !self.is_win(max) {
            max -= 1;
        }

        println!(
            "time:{}  distance:{}  {min}..={max}",
            self.time, self.distance
        );
        min..=max
    }
}

pub struct Day06 {
    times: String,
    distances: String,
}

impl Day06 {
    pub fn new() -> Self {
        Self {
            times: String::new(),
            distances: String::new(),
        }
    }

    fn get_races(times: &str, distances: &str) -> Vec<Race> {
        let times = times
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let distances = distances
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(times.len(), distances.len());
        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(time, distance)| Race {
                time: *time,
                distance: *distance,
            })
            .collect::<Vec<_>>();
        println!("'{:?}'", races);
        races
    }
}

impl Runner for Day06 {
    fn parse(&mut self, path: &str) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        let lines: Vec<&str> = lines.iter().collect();
        assert_eq!(lines.len(), 2);
        self.times = lines[0][10..].trim().to_string();
        self.distances = lines[1][10..].trim().to_string();
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let races = Self::get_races(&self.times, &self.distances);
        let mut ans = 1;
        for race in races.iter() {
            let wins = race.wins();
            ans *= wins.end() - wins.start() + 1;
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let times: String = String::from_iter(self.times.chars().filter(|c| !c.is_whitespace()));
        let distances: String =
            String::from_iter(self.distances.chars().filter(|c| !c.is_whitespace()));
        let races = Self::get_races(&times, &distances);
        let mut ans = 1;
        for race in races.iter() {
            let wins = race.wins();
            ans *= wins.end() - wins.start() + 1;
        }
        Ok(ans.into())
    }
}
