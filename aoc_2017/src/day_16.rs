#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Spin(isize),
    Exchange(usize, usize),
    Partner(usize, usize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('s') {
            Ok(Instruction::Spin(s.parse()?))
        } else if let Some(s) = s.strip_prefix('x') {
            let parts: Vec<&str> = s.split("/").collect();
            Ok(Instruction::Exchange(parts[0].parse()?, parts[1].parse()?))
        } else if let Some(s) = s.strip_prefix('p') {
            let chars: Vec<char> = s.chars().collect();
            Ok(Instruction::Partner(
                (chars[0] as u8 - b'a') as usize,
                (chars[2] as u8 - b'a') as usize,
            ))
        } else {
            Err(Error::InvalidInput(s.to_string()))
        }
    }
}

#[derive(Default)]
pub struct Day16 {
    instructions: Vec<Instruction>,
    head: usize,
    programs: Vec<usize>,
    positions: Vec<usize>,
}

impl Day16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn dance(&mut self) {
        let num_positions: isize = self.positions.len() as isize;
        for inst in self.instructions.iter() {
            match inst {
                Instruction::Spin(c) => {
                    self.head = (self.head as isize - c).rem_euclid(num_positions) as usize;
                }
                Instruction::Exchange(a, b) => {
                    let a = (a + self.head) % num_positions as usize;
                    let b = (b + self.head) % num_positions as usize;
                    let p_at_a = self.positions[a];
                    let p_at_b = self.positions[b];
                    self.programs[p_at_a] = b;
                    self.programs[p_at_b] = a;
                    self.positions[b] = p_at_a;
                    self.positions[a] = p_at_b;
                }
                Instruction::Partner(p_a, p_b) => {
                    let a = self.programs[*p_a];
                    let b = self.programs[*p_b];
                    self.programs[*p_a] = b;
                    self.programs[*p_b] = a;
                    self.positions[b] = *p_a;
                    self.positions[a] = *p_b;
                }
            }
        }
    }
}

impl std::fmt::Display for Day16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.positions.len();
        for i in 0..len {
            let idx = (i + self.head) % len;
            write!(f, "{}", (self.positions[idx] as u8 + b'a') as char)?;
        }
        Ok(())
    }
}

impl Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines[0].split(',') {
            self.instructions.push(line.parse()?);
        }

        if self.instructions.len() < 10 {
            self.positions.extend(0..5);
            self.programs.extend(0..5);
        } else {
            self.positions.extend(0..16);
            self.programs.extend(0..16);
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

impl Day16 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.dance();
        Ok(self.to_string().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut seen = Vec::new();
        seen.push(self.to_string());
        const ITERS: usize = 1_000_000_000;
        for step in 1..ITERS {
            self.dance();
            let cur = self.to_string();
            if let Some(start) = seen.iter().position(|s| s == &cur) {
                let cycle_len = step - start;
                let idx = start + ((ITERS - step) % cycle_len);
                return Ok(seen[idx].clone().into());
            }
            seen.push(cur);
        }
        Err(Error::Unsolved)
    }
}
