#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

struct Inst {
    from: usize,
    to: usize,
    count: usize,
}

pub struct Day05 {
    piles: Vec<Vec<char>>,
    insts: Vec<Inst>,
}

impl Day05 {
    pub fn new() -> Self {
        Self {
            piles: Vec::new(),
            insts: Vec::new(),
        }
    }
}

impl Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let chars: Vec<char> = line.chars().collect();
            for (pos, c) in chars.iter().skip(1).step_by(4).enumerate() {
                while self.piles.len() <= pos {
                    self.piles.push(Vec::new());
                }
                if *c >= 'A' && *c <= 'Z' {
                    self.piles[pos].push(*c);
                }
            }
        }
        for pile in self.piles.iter_mut() {
            pile.reverse();
        }

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let count = parts[1].parse()?;
            let from = parts[3].parse::<usize>()? - 1;
            let to = parts[5].parse::<usize>()? - 1;
            self.insts.push(Inst { from, to, count });
        }
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

impl Day05 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut holding = Vec::new();
        for inst in self.insts.iter() {
            let start = self.piles[inst.from].len() - inst.count;
            let crates = self.piles[inst.from].drain(start..);
            holding.extend(crates.rev());
            self.piles[inst.to].append(&mut holding);
        }
        Ok(self
            .piles
            .iter_mut()
            .map(|pile| pile.pop().unwrap())
            .collect::<String>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut holding = Vec::new();
        for inst in self.insts.iter() {
            let start = self.piles[inst.from].len() - inst.count;
            let crates = self.piles[inst.from].drain(start..);
            holding.extend(crates);
            self.piles[inst.to].append(&mut holding);
        }
        Ok(self
            .piles
            .iter_mut()
            .map(|pile| pile.pop().unwrap())
            .collect::<String>()
            .into())
    }
}
