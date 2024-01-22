#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day08 {
    layers: Vec<Vec<Vec<u8>>>,
}

impl Day08 {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
}

impl Runner for Day08 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        let chars: Vec<char> = lines[0].chars().collect();
        for layer in chars.chunks(25 * 6) {
            self.layers.push(
                layer
                    .chunks(25)
                    .map(|line| line.iter().map(|c| *c as u8 - b'0').collect())
                    .collect(),
            );
        }
        Ok(())
    }

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
        let mut ans = String::with_capacity(26 * 6);
        for y in 0..6 {
            for x in 0..25 {
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
