#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

struct Digit(u8);

impl Digit {
    fn count_on(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl std::fmt::Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:07b} ({})", self.0, self.count_on())
    }
}

#[derive(Default)]
struct Display {
    unique: Vec<Digit>,
    output: Vec<Digit>,
}

impl Display {
    fn decode(&self) -> usize {
        const SEG_A: usize = 0;
        const SEG_B: usize = 1;
        const SEG_C: usize = 2;
        const SEG_D: usize = 3;
        const SEG_E: usize = 4;
        const SEG_F: usize = 5;
        const SEG_G: usize = 6;

        let mut decoded = [0u8; 10];
        let mut segments = [0; 7];
        for unique in self.unique.iter() {
            match unique.count_on() {
                2 => decoded[1] = unique.0,
                3 => decoded[7] = unique.0,
                4 => decoded[4] = unique.0,
                7 => decoded[8] = unique.0,
                _ => {}
            }
        }

        // 7 has exactly 1 more than 1.
        segments[SEG_A] = decoded[7] & !decoded[1];

        // 0, 6, and 9 share f with 1.
        segments[SEG_F] = decoded[1];
        self.unique
            .iter()
            .filter(|d| d.count_on() == 6)
            .for_each(|d| segments[SEG_F] &= d.0);

        // That means the left over in 1 is c
        segments[SEG_C] = decoded[1] & !segments[SEG_F];

        // 3 has 2 additional segments than 7
        self.unique.iter().for_each(|d| {
            let diff = d.0 & !decoded[7];
            let same = d.0 & decoded[7];
            if diff.count_ones() == 2 && same == decoded[7] {
                // g is the one that is not in 4, d is the one that is in 4
                segments[SEG_G] = diff & !decoded[4];
                segments[SEG_D] = diff & decoded[4];
                decoded[3] = d.0;
            }
        });

        // b is the last unknown in 4
        segments[SEG_B] = decoded[4] & !(segments.iter().fold(0, |a, b| a | b));

        // e is the last unknown in 8
        segments[SEG_E] = decoded[8] & !(segments.iter().fold(0, |a, b| a | b));

        decoded[0] = 0b111_1111 & !segments[SEG_D];
        decoded[2] = 0b111_1111 & !(segments[SEG_B] | segments[SEG_F]);
        decoded[5] = 0b111_1111 & !(segments[SEG_C] | segments[SEG_E]);
        decoded[6] = 0b111_1111 & !segments[SEG_C];
        decoded[9] = 0b111_1111 & !segments[SEG_E];

        self.output.iter().fold(0, |v, d| {
            (v * 10) + decoded.iter().position(|d2| *d2 == d.0).unwrap()
        })
    }
}

impl FromStr for Display {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((unique, output)) = s.split_once(" | ") {
            let mut display = Display::default();
            for unique in unique.split_whitespace() {
                display.unique.push(unique.parse()?);
            }
            for output in output.split_whitespace() {
                display.output.push(output.parse()?);
            }
            Ok(display)
        } else {
            Err(Error::InvalidInput(format!("{s:?} is invalid")))
        }
    }
}

impl FromStr for Digit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = 0u8;
        for c in s.chars() {
            let idx = c as u8 - b'a';
            if idx > 7 {
                return Err(Error::InvalidInput(format!(
                    "{s:?} has invalid segment {c:?}"
                )));
            }
            segments |= 1 << idx;
        }
        Ok(Self(segments))
    }
}

pub struct Day08 {
    displays: Vec<Display>,
}

impl Day08 {
    pub fn new() -> Self {
        Self {
            displays: Vec::new(),
        }
    }
}

impl Runner for Day08 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::ALL)?;
        for line in lines.iter() {
            self.displays.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let numbers = [2, 3, 4, 7];

        Ok(self
            .displays
            .iter()
            .fold(0, |ans, d| {
                ans + d
                    .output
                    .iter()
                    .filter(|o| numbers.contains(&o.count_on()))
                    .count()
            })
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .displays
            .iter()
            .map(|d| d.decode())
            .sum::<usize>()
            .into())
    }
}
