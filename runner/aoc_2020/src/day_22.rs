#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::VecDeque;

pub struct Day22 {
    players: [VecDeque<u8>; 2],
}

impl Day22 {
    pub fn new() -> Self {
        Self {
            players: std::array::from_fn(|_| VecDeque::with_capacity(25)),
        }
    }
}

impl Runner for Day22 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::REMOVE_EMPTY)?;
        let mut player: usize = 0;

        for line in lines.iter() {
            if let Some(p) = line.strip_prefix("Player ") {
                player = p[..p.len() - 1].parse::<usize>()? - 1;
            } else {
                self.players[player].push_front(line.parse()?);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        for (i, p) in self.players.iter().enumerate() {
            println!("{i} ({}): {:?}", p.len(), p);
        }
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }
}
