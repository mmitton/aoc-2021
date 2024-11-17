#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day01 {
    masses: Vec<isize>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { masses: Vec::new() }
    }

    pub fn fuel_required(mass: isize) -> isize {
        mass / 3 - 2
    }

    pub fn total_fuel_required(mass: isize) -> isize {
        let fuel = Self::fuel_required(mass);
        if fuel < 0 {
            return 0;
        }
        fuel + Self::total_fuel_required(fuel)
    }
}

impl Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
            self.masses.push(line.parse()?);
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

impl Day01 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .masses
            .iter()
            .copied()
            .map(Self::fuel_required)
            .sum::<isize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .masses
            .iter()
            .copied()
            .map(Self::total_fuel_required)
            .sum::<isize>()
            .into())
    }
}
