#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day25 {
    locks: Vec<[usize; 5]>,
    keys: Vec<[usize; 5]>,
}

impl Day25 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .locks
            .iter()
            .map(|lock| {
                self.keys
                    .iter()
                    .filter(|key| !lock.iter().zip(key.iter()).any(|(l, k)| l + k > 5))
                    .count()
            })
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::REMOVE_EMPTY)?;
        for lines in lines.chunks(7) {
            let (lines, is_key): (Vec<Vec<char>>, bool) = if lines[0] == "#####" {
                (
                    lines.iter().skip(1).map(|l| l.chars().collect()).collect(),
                    false,
                )
            } else {
                (
                    lines
                        .iter()
                        .rev()
                        .skip(1)
                        .map(|l| l.chars().collect())
                        .collect(),
                    true,
                )
            };

            let mut heights = [0; 5];
            for x in 0..5 {
                for y in (0..6).rev() {
                    if lines[y][x] == '#' {
                        heights[x] = y + 1;
                        break;
                    }
                }
            }

            if is_key {
                self.keys.push(heights);
            } else {
                self.locks.push(heights);
            }
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            _ => Err(Error::Skipped),
        }
    }
}
