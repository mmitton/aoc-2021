#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day06 {
    chars: Vec<u32>,
}

impl Day06 {
    pub fn new() -> Self {
        Self { chars: Vec::new() }
    }
}

fn find_marker<const N: usize>(chars: &[u32]) -> usize {
    let mut window: [u32; N] = std::array::from_fn(|i| chars[i]);

    for i in N..chars.len() {
        window[i % N] = chars[i];

        if window.iter().fold(0, |acc, c| acc | c).count_ones() as usize == N {
            return i + 1;
        }
    }

    0
}

impl Runner for Day06 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        self.chars
            .extend(lines[0].chars().map(|c| 1u32 << (c as u8 - b'a')));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(find_marker::<4>(&self.chars).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(find_marker::<14>(&self.chars).into())
    }
}
