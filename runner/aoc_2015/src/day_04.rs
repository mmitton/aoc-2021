#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner, MD5,
};
use std::fmt::Write;

#[derive(Default)]
pub struct Day04 {
    key: String,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scan<F>(&mut self, f: F) -> Result<usize, Error>
    where
        F: Fn(u8) -> bool,
    {
        let len = self.key.len();
        for i in 0.. {
            write!(self.key, "{i}")?;
            let digest = MD5::digest(self.key.as_bytes());
            self.key.truncate(len);
            if digest[0] == 0 && digest[1] == 0 && f(digest[2]) {
                return Ok(i);
            }
        }
        unreachable!();
    }
}

impl Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.key.write_str(lines[0].as_str())?;
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.scan(|v| v & 0xf0 == 0)?.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.scan(|v| v == 0)?.into())
    }
}
