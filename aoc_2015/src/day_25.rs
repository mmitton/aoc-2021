#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day25 {
    row: usize,
    col: usize,
}

impl Day25 {
    pub fn new() -> Self {
        Self::default()
    }

    fn calc(&self) -> usize {
        let mut cr = 1;
        let mut cc = 1;
        let mut code = 20151125;
        while cr != self.row || cc != self.col {
            code = (code * 252533) % 33554393;
            if cr == 1 {
                cr = cc + 1;
                cc = 1;
            } else {
                cr -= 1;
                cc += 1;
            }
        }

        code
    }
}

impl Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        let line = lines[0].replace(
            "To continue, please consult the code grid in the manual.  Enter the code at row ",
            "",
        );
        let line = line.replace(", column", "");
        let line = line.replace('.', "");
        let parts: Vec<&str> = line.split_whitespace().collect();
        self.row = parts[0].parse()?;
        self.col = parts[1].parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.calc().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Skipped)
    }
}
