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
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.chars
            .extend(lines[0].chars().map(|c| 1u32 << (c as u8 - b'a')));
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

impl Day06 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(find_marker::<4>(&self.chars).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(find_marker::<14>(&self.chars).into())
    }
}
