#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day16 {
    initial_state: String,
}

impl Day16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn checksum(&self, len: usize) -> String {
        let mut data = self.initial_state.clone();
        while data.len() < len {
            let chars: Vec<char> = data.chars().collect();
            data.push('0');
            for c in chars.iter().rev() {
                match c {
                    '1' => data.push('0'),
                    '0' => data.push('1'),
                    _ => unreachable!(),
                }
            }
        }
        let data = data[0..len].to_string();
        let mut checksum = data.clone();
        loop {
            let mut new_checksum = String::new();
            for i in (0..checksum.len()).step_by(2) {
                match &checksum[i..i + 2] {
                    "00" | "11" => new_checksum.push('1'),
                    _ => new_checksum.push('0'),
                }
            }
            checksum = new_checksum;

            if checksum.len() % 2 == 1 {
                break;
            }
        }
        checksum
    }
}

impl Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.initial_state.push_str(&lines[0]);
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

impl Day16 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let len = if self.initial_state.len() == 5 {
            20
        } else {
            272
        };
        Ok(self.checksum(len).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.checksum(35651584).into())
    }
}
