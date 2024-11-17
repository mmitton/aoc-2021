#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day05 {
    seats: [u8; 128],
}

impl Day05 {
    pub fn new() -> Self {
        Self { seats: [0; 128] }
    }
}

impl Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        fn parse(inst: &str, mut low: usize, mut high: usize) -> usize {
            for c in inst.chars() {
                let half_width = (high - low + 1) / 2;
                match c {
                    'F' | 'L' => high -= half_width,
                    'B' | 'R' => low += half_width,
                    _ => unreachable!(),
                }
                // println!("{c} {half_width} {low} {high}");
            }
            low
        }

        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let row = parse(&line[0..7], 0, 127);
            let seat = parse(&line[7..], 0, 7);
            // println!("{line} => {row} {seat}");
            self.seats[row] |= 1 << seat;
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
        for (row, seats) in self.seats.iter().enumerate().rev() {
            if *seats != 0 {
                let seat = 7 - seats.leading_zeros() as usize;
                return Ok(((row * 8) + seat).into());
            }
        }
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for (row, seats) in self.seats.iter().enumerate() {
            if seats.count_ones() == 7 {
                let seat = 7 - seats.leading_ones() as usize;
                return Ok(((row * 8) + seat).into());
            }
        }
        Err(Error::Unsolved)
    }
}
