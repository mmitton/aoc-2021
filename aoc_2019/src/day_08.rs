#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day08 {
    width: usize,
    height: usize,
    layers: Vec<Vec<Vec<u8>>>,
}

impl Day08 {
    pub fn new() -> Self {
        Self {
            width: 25,
            height: 6,
            layers: Vec::new(),
        }
    }
}

impl Runner for Day08 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let input = match lines.len() {
            1 => lines[0].as_str(),
            2 => {
                let (w, h) = lines[0].split_once('x').unwrap();
                self.width = w.parse()?;
                self.height = h.parse()?;
                lines[1].as_str()
            }
            _ => panic!("Input needs to be 1 or 2 lines"),
        };
        let chars: Vec<char> = input.chars().collect();
        for layer in chars.chunks(self.width * self.height) {
            self.layers.push(
                layer
                    .chunks(self.width)
                    .map(|line| line.iter().map(|c| *c as u8 - b'0').collect())
                    .collect(),
            );
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

impl Day08 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;
        let mut min_zeros = usize::MAX;
        for layer in self.layers.iter() {
            let mut counts = [0; 10];
            for line in layer.iter() {
                for num in line.iter() {
                    counts[*num as usize] += 1;
                }
            }
            if counts[0] < min_zeros {
                ans = counts[1] * counts[2];
                min_zeros = counts[0];
            }
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut ans = String::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                for layer in self.layers.iter() {
                    match layer[y][x] {
                        0 => {
                            ans.push(' ');
                            break;
                        }
                        1 => {
                            ans.push('#');
                            break;
                        }
                        _ => {}
                    }
                }
            }
            ans.push('\n');
        }
        Ok(ans.into())
    }
}
