#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
enum OnOff {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    on_off: OnOff,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

struct Grid(Vec<usize>);

impl Grid {
    fn new() -> Self {
        Self(vec![0; 1000 * 1000])
    }

    fn process<F>(&mut self, instructions: &[Instruction], f: F)
    where
        F: Fn(&OnOff, usize) -> usize,
    {
        for instruction in instructions.iter() {
            for y in instruction.y1..=instruction.y2 {
                for x in instruction.x1..=instruction.x2 {
                    let idx = ((y * 1000) + x) as usize;
                    self.0[idx] = f(&instruction.on_off, self.0[idx]);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Day06 {
    instructions: Vec<Instruction>,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day06 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        for line in lines.iter() {
            let (on_off, line) = if line.starts_with("turn on") {
                (OnOff::On, &line[8..])
            } else if line.starts_with("turn off") {
                (OnOff::Off, &line[9..])
            } else {
                (OnOff::Toggle, &line[7..])
            };

            let line = line.replace(" through ", " ");
            let line = line.replace(",", " ");
            let parts: Vec<&str> = line.split(" ").collect();

            self.instructions.push(Instruction {
                on_off,
                x1: parts[0].parse()?,
                y1: parts[1].parse()?,
                x2: parts[2].parse()?,
                y2: parts[3].parse()?,
            });
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

impl Day06 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut grid = Grid::new();
        grid.process(&self.instructions, |on_off, v| match on_off {
            OnOff::On => 1,
            OnOff::Off => 0,
            OnOff::Toggle => 1 - v,
        });
        Ok(grid.0.iter().sum::<usize>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut grid = Grid::new();
        grid.process(&self.instructions, |on_off, v| match on_off {
            OnOff::On => v + 1,
            OnOff::Off => v.saturating_sub(1),
            OnOff::Toggle => v + 2,
        });
        Ok(grid.0.iter().sum::<usize>().into())
    }
}
