#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, shift: usize },
    RotateCol { col: usize, shift: usize },
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix("rect ") {
            if let Some((width, height)) = rest.split_once('x') {
                Ok(Self::Rect {
                    width: width.parse()?,
                    height: height.parse()?,
                })
            } else {
                Err(Error::InvalidInput(s.into()))
            }
        } else if let Some(rest) = s.strip_prefix("rotate column x=") {
            if let Some((col, shift)) = rest.split_once(" by ") {
                Ok(Self::RotateCol {
                    col: col.parse()?,
                    shift: shift.parse()?,
                })
            } else {
                Err(Error::InvalidInput(s.into()))
            }
        } else if let Some(rest) = s.strip_prefix("rotate row y=") {
            if let Some((row, shift)) = rest.split_once(" by ") {
                Ok(Self::RotateRow {
                    row: row.parse()?,
                    shift: shift.parse()?,
                })
            } else {
                Err(Error::InvalidInput(s.into()))
            }
        } else {
            Err(Error::InvalidInput(s.into()))
        }
    }
}

#[derive(Default)]
pub struct Day08 {
    instructions: Vec<Instruction>,
    grid: Vec<Vec<bool>>,
}

impl std::fmt::Display for Day08 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for c in row.iter() {
                write!(f, "{}", if *c { '#' } else { ' ' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }

    fn process(&mut self) {
        let mut next_grid = self.grid.clone();
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Rect { width, height } => {
                    next_grid
                        .iter_mut()
                        .take(*height)
                        .for_each(|r| r.iter_mut().take(*width).for_each(|c| *c = true));
                }
                Instruction::RotateRow { row, shift } => {
                    let row = *row;
                    let len = self.grid[row].len();
                    for c in 0..len {
                        next_grid[row][(c + shift) % len] = self.grid[row][c];
                    }
                }
                Instruction::RotateCol { col, shift } => {
                    let col = *col;
                    let len = self.grid.len();
                    for r in 0..len {
                        next_grid[(r + shift) % len][col] = self.grid[r][col];
                    }
                }
            }

            self.grid
                .iter_mut()
                .zip(next_grid.iter())
                .for_each(|(r, nr)| {
                    r.iter_mut().zip(nr.iter()).for_each(|(c, nc)| *c = *nc);
                });
        }
    }
}

impl Runner for Day08 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.instructions.push(line.parse()?);
        }
        let (width, height) = if self.instructions.len() == 4 {
            (7, 3)
        } else {
            (50, 6)
        };

        self.grid.extend((0..height).map(|_| vec![false; width]));
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

impl Day08 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.process();
        Ok(self
            .grid
            .iter()
            .map(|row| row.iter().filter(|c| **c).count())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.process();
        Ok(self.to_string().into())
    }
}
