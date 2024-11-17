#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day20 {
    target: usize,
}

impl Day20 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_house(&self, part2: bool) -> usize {
        // Define upper limit of search based on the magnatude of the target
        let total_houses = 10usize.pow((self.target as f64).log10() as u32 - 1);

        let mut houses = vec![10; total_houses];
        let mut min_house = usize::MAX;
        for elf in 2..total_houses {
            if elf > min_house {
                break;
            }
            if !part2 {
                for house in (elf..total_houses).step_by(elf) {
                    houses[house] += 10 * elf;
                    if houses[house] >= self.target {
                        min_house = min_house.min(house);
                    }
                }
            } else {
                for house in (elf..total_houses).step_by(elf).take(50) {
                    houses[house] += 11 * elf;
                    if houses[house] >= self.target {
                        min_house = min_house.min(house);
                    }
                }
            }
        }

        min_house
    }
}

impl Runner for Day20 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.target = lines[0].parse()?;
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

impl Day20 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_house(false).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_house(true).into())
    }
}
