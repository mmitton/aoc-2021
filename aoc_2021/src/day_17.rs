#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day17 {
    min: (isize, isize),
    max: (isize, isize),
}

impl Day17 {
    pub fn new() -> Self {
        Self {
            min: (0, 0),
            max: (0, 0),
        }
    }

    fn shot_good(&self, mut dx: isize, mut dy: isize) -> bool {
        let min_y = self.min.1.min(self.max.1);

        let mut x = 0;
        let mut y = 0;

        while y >= min_y {
            x += dx;
            y += dy;

            dy -= 1;
            if dx != 0 {
                dx -= 1;
            }

            if (self.min.0..=self.max.0).contains(&x) && (self.min.1..=self.max.1).contains(&y) {
                return true;
            }
        }
        false
    }
}

impl Runner for Day17 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        let line = &lines[0][15..];
        let line = line.replace("..", " ").replace(", y=", " ");
        let parts = line.split(" ").collect::<Vec<&str>>();
        self.min.0 = parts[0].parse()?;
        self.max.0 = parts[1].parse()?;
        self.min.1 = parts[2].parse()?;
        self.max.1 = parts[3].parse()?;

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        println!("{:?} {:?}", self.min, self.max);
        let max_y = self.min.1.min(self.max.1).abs() - 1;
        Ok((1..=max_y).sum::<isize>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut shots = 0;

        // Find x bounds
        let max_x = self.min.0.max(self.max.0).abs();
        let mut min_x = self.min.0.min(self.max.0);
        for x in 1..max_x {
            let dx: isize = (1..=x).sum();
            if dx > min_x {
                min_x = x;
                break;
            }
        }

        let min_y = self.min.1.min(self.max.1);
        let max_y = self.min.1.min(self.max.1).abs() - 1;
        println!("x: {min_x}..{max_x} y: {min_y}..{max_y}");

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.shot_good(x, y) {
                    shots += 1;
                }
            }
        }

        Ok(shots.into())
    }
}
