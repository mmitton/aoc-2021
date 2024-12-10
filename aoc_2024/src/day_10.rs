#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};
use std::collections::VecDeque;

#[derive(Default)]
pub struct Day10 {
    map: Vec<Vec<u8>>,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }

    fn _print(&self) {
        for row in self.map.iter() {
            for elevation in row.iter() {
                if *elevation == u8::MAX {
                    print!(".");
                } else {
                    print!("{elevation}");
                }
            }
            println!();
        }
    }

    fn map_trailhead(&self, x: usize, y: usize) -> (usize, usize) {
        let mut work = VecDeque::new();
        work.push_front(((x, y), 0));

        let mut trails = 0;
        let mut end_points = HashSet::default();
        while let Some(((x, y), elevation)) = work.pop_front() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if let Some(next_elevation) = self.map.get(ny).and_then(|row| row.get(nx)) {
                    let next_elevation = *next_elevation;
                    if next_elevation == elevation + 1 {
                        if next_elevation == 9 {
                            trails += 1;
                            end_points.insert((nx, ny));
                        } else {
                            work.push_back(((nx, ny), next_elevation));
                        }
                    }
                }
            }
        }
        (end_points.len(), trails)
    }

    fn find_trailheads(&self) -> (usize, usize) {
        let mut total_end_points = 0;
        let mut total_trails = 0;
        for (y, row) in self.map.iter().enumerate() {
            for (x, elevation) in row.iter().enumerate() {
                if *elevation == 0 {
                    let (ep, t) = self.map_trailhead(x, y);
                    total_end_points += ep;
                    total_trails += t;
                }
            }
        }
        (total_end_points, total_trails)
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.find_trailheads().0.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.find_trailheads().1.into())
    }
}

impl helper::Runner for Day10 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let mut row = Vec::new();
            for c in line.chars() {
                if c.is_ascii_digit() {
                    row.push(c as u8 - b'0');
                } else {
                    row.push(u8::MAX);
                }
            }
            self.map.push(row);
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
