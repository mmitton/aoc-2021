#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::cmp::Ordering;

pub struct Day03 {
    numbers: Vec<usize>,
    width: usize,
}

fn calc_bit(numbers: &[usize], n: usize) -> (usize, usize, usize) {
    let (mut ones, mut zeros) = (0, 0);
    let bit = 1 << n;
    numbers.iter().for_each(|n| {
        if n & bit == 0 {
            zeros += 1;
        } else {
            ones += 1
        }
    });

    match ones.cmp(&zeros) {
        Ordering::Equal => (bit, 0, bit),
        Ordering::Greater => (bit, 0, bit),
        Ordering::Less => (0, bit, bit),
    }
}

impl Day03 {
    pub fn new() -> Self {
        Self {
            numbers: Vec::new(),
            width: 0,
        }
    }

    fn gamma_epsilon(&self) -> (usize, usize) {
        let (mut gamma, mut epsilon) = (0, 0);
        for bit in 0..self.width {
            let (g, e, _) = calc_bit(&self.numbers, bit);
            gamma |= g;
            epsilon |= e;
        }

        (gamma, epsilon)
    }

    fn oxygen_co2(&self) -> (usize, usize) {
        fn reduce(mut numbers: Vec<usize>, width: usize, most_common: bool) -> usize {
            for bit in (0..width).rev() {
                let (most, least, bit) = calc_bit(&numbers, bit);
                let v = if most_common { most } else { least };
                numbers.retain(|n| n & bit == v);
                assert!(!numbers.is_empty());

                if numbers.len() == 1 {
                    return numbers[0];
                }
            }
            unreachable!()
        }

        let oxygen = reduce(self.numbers.clone(), self.width, true);
        let co2 = reduce(self.numbers.clone(), self.width, false);

        (oxygen, co2)
    }
}

impl Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        self.width = lines[0].len();
        self.numbers
            .extend(lines.iter().map(|l| usize::from_str_radix(l, 2).unwrap()));
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

impl Day03 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let (gamma, epsilon) = self.gamma_epsilon();
        Ok((gamma * epsilon).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let (oxygen, co2) = self.oxygen_co2();
        Ok((oxygen * co2).into())
    }
}
