#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone, Default)]
struct Number {
    digits: Vec<u8>,
}

impl Number {
    fn push(&mut self, digit: u8, num: usize) {
        let mut mag = 1;
        while num / mag != 0 {
            mag *= 10;
        }
        mag /= 10;

        let mut num = num;
        loop {
            self.digits.push((num / mag) as u8);
            num %= mag;

            if mag == 1 {
                break;
            }
            mag /= 10;
        }

        self.digits.push(digit);
    }

    fn next(&self) -> Self {
        let mut next = Number {
            digits: Vec::with_capacity(self.digits.len() * 2),
        };

        let mut cur = self.digits[0];
        let mut cur_len = 1;
        for c in self.digits[1..].iter().copied() {
            if c != cur {
                next.push(cur, cur_len);
                cur = c;
                cur_len = 1;
            } else {
                cur_len += 1;
            }
        }
        next.push(cur, cur_len);

        next
    }
}

#[derive(Default)]
pub struct Day10 {
    number: Number,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day10 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        for c in lines[0].chars() {
            self.number.digits.push(c as u8 - b'0');
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

impl Day10 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut next = self.number.clone();
        for _ in 0..40 {
            next = next.next();
        }
        Ok(next.digits.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut next = self.number.clone();
        for _ in 0..50 {
            next = next.next();
        }
        Ok(next.digits.len().into())
    }
}
