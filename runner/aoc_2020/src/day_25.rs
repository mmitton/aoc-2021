#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug, Default)]
struct Key {
    public: usize,
    loop_size: usize,
}

pub struct Day25 {
    card: Key,
    door: Key,
}

impl Day25 {
    pub fn new() -> Self {
        Self {
            card: Key::default(),
            door: Key::default(),
        }
    }
}

impl Runner for Day25 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 2);
        self.card.public = lines[0].parse()?;
        self.door.public = lines[1].parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut v = 1;
        let mut loops = 0;
        while self.card.loop_size == 0 || self.door.loop_size == 0 {
            v *= 7;
            v = v % 20201227;
            loops += 1;

            if self.card.loop_size == 0 && v == self.card.public {
                self.card.loop_size = loops;
            }
            if self.door.loop_size == 0 && v == self.door.public {
                self.door.loop_size = loops;
            }
        }

        let mut v = 1;
        for _ in 0..self.door.loop_size {
            v *= self.card.public;
            v = v % 20201227;
        }
        Ok(v.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Skipped)
    }
}
