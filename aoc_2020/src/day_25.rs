#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day25 {
    card: usize,
    door: usize,
}

impl Day25 {
    pub fn new() -> Self {
        Self { card: 0, door: 0 }
    }
}

impl Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 2);
        self.card = lines[0].parse()?;
        self.door = lines[1].parse()?;
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day25 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut door_public = 1;
        let mut card_private = 1;
        for _ in 1.. {
            door_public *= 7;
            door_public %= 20201227;

            card_private *= self.card;
            card_private %= 20201227;

            if door_public == self.door {
                break;
            }
        }

        Ok(card_private.into())
    }
}
