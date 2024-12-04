#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day04 {
    map: Vec<Vec<char>>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        self.map.get(y as usize)?.get(x as usize).copied()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut seen = 0;
        for y in 0..self.map.len() {
            let y = y as isize;
            for x in 0..self.map[0].len() {
                let x = x as isize;
                if self.get(x, y) == Some('X') {
                    'dir_search: for dir in [
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (-1, 0),
                        (1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ] {
                        for (d, mas) in ['X', 'M', 'A', 'S'].iter().copied().enumerate().skip(1) {
                            let x = x + (d as isize * dir.0);
                            let y = y + (d as isize * dir.1);
                            if self.get(x, y) != Some(mas) {
                                continue 'dir_search;
                            }
                        }
                        seen += 1;
                    }
                }
            }
        }
        Ok(seen.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut seen = 0;
        for y in 0..self.map.len() {
            let y = y as isize;
            for x in 0..self.map[0].len() {
                let x = x as isize;
                macro_rules! check {
                    ($x0:expr, $y0:expr, $x1:expr, $y1:expr) => {{
                        match (self.get($x0, $y0), self.get($x1, $y1)) {
                            (Some('M'), Some('S')) | (Some('S'), Some('M')) => true,
                            _ => false,
                        }
                    }};
                }
                if self.get(x, y) == Some('A')
                    && check!(x - 1, y - 1, x + 1, y + 1)
                    && check!(x + 1, y - 1, x - 1, y + 1)
                {
                    seen += 1;
                }
            }
        }
        Ok(seen.into())
    }
}

impl helper::Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
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
