#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day01 {
    moves: Vec<isize>,
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        for c in lines[0].chars() {
            let n = match c {
                '(' => 1,
                ')' => -1,
                _ => return Err(Error::InvalidInput(format!("{c}"))),
            };
            self.moves.push(n);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.moves.iter().sum::<isize>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut floor = 0;
        for (i, n) in self.moves.iter().enumerate() {
            floor += n;
            if floor == -1 {
                return Ok((i + 1).into());
            }
        }
        Err(Error::Unsolved)
    }
}
