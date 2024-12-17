#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};
use std::str::FromStr;

#[derive(Debug)]
struct Robot {
    pos: Point2D<isize>,
    vel: Point2D<isize>,
}

impl Robot {
    fn pos(&self, at: isize, tiles: Point2D<isize>) -> Point2D<isize> {
        let mut pos = self.pos + self.vel.scale(at);
        pos.x = pos.x.rem_euclid(tiles.x);
        pos.y = pos.y.rem_euclid(tiles.y);

        pos
    }
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_s = s.replace(['p', 'v', '='], "");
        let Some((p, v)) = clean_s.split_once(' ') else {
            return Err(Error::InvalidInput(s.into()));
        };
        let Some((px, py)) = p.split_once(',') else {
            return Err(Error::InvalidInput(s.into()));
        };
        let Some((vx, vy)) = v.split_once(',') else {
            return Err(Error::InvalidInput(s.into()));
        };

        Ok(Self {
            pos: Point2D::new(px.parse()?, py.parse()?),
            vel: Point2D::new(vx.parse()?, vy.parse()?),
        })
    }
}

#[derive(Default)]
pub struct Day14 {
    robots: Vec<Robot>,
    tiles: Point2D<isize>,
}

impl Day14 {
    pub fn new() -> Self {
        Self::default()
    }

    fn safety_factor(&self, t: isize) -> usize {
        let mut quads = [0usize; 4];
        let cx = self.tiles.x / 2;
        let cy = self.tiles.y / 2;
        for robot in self.robots.iter() {
            let pos = robot.pos(t, self.tiles);

            if pos.x == cx || pos.y == cy {
                continue;
            }
            let qx = if pos.x > cx { 1 } else { 0 };
            let qy = if pos.y > cy { 2 } else { 0 };
            let q = qx + qy;

            quads[q] += 1;
        }

        quads.iter().product()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.safety_factor(100).into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        for t in 0..20_000 {
            if self.safety_factor(t) < 100000000 {
                return Ok(t.into());
            }
        }
        Err(Error::Unsolved)
    }
}

impl helper::Runner for Day14 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.robots.push(line.parse()?);
        }

        println!("{}", self.robots.len());
        self.tiles = if self.robots.len() == 12 {
            Point2D::new(11, 7)
        } else {
            Point2D::new(101, 103)
        };
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
