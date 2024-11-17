#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::ops::Range;

pub struct Day16 {
    signal: Vec<u8>,
}

impl Day16 {
    pub fn new() -> Self {
        Self { signal: Vec::new() }
    }
}

impl Day16 {
    fn phase_1(from: &[u8], to: &mut [u8]) {
        assert_eq!(from.len(), to.len());

        for (i, to) in to.iter_mut().enumerate() {
            let mut val = 0isize;

            let mut add = true;
            let mut j = i;
            while j < from.len() {
                let mut chunk = i + 1;
                while j < from.len() && chunk > 0 {
                    if add {
                        val += from[j] as isize;
                    } else {
                        val -= from[j] as isize;
                    }
                    j += 1;
                    chunk -= 1;
                }
                add = !add;
                j += i + 1;
            }

            *to = (val.abs() % 10) as u8;
        }
    }

    fn phase_2(skip: usize, from: &[u8], to: &mut [u8]) {
        assert_eq!(from.len(), to.len());

        let mut num = 0usize;
        for i in (skip..from.len()).rev() {
            num += from[i] as usize;
            to[i] = (num % 10) as u8;
        }
    }

    fn get_num(&self, range: Range<usize>) -> usize {
        let mut num = 0usize;
        for v in self.signal[range].iter() {
            num = num * 10 + *v as usize;
        }
        num
    }
}

impl Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert!(lines.len() == 1);
        self.signal.extend(lines[0].chars().map(|c| c as u8 - b'0'));
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
        let mut to = vec![0; self.signal.len()];
        for _ in 1..=100 {
            Self::phase_1(&self.signal, &mut to);
            std::mem::swap(&mut self.signal, &mut to);
        }
        Ok(self.get_num(0..8).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        const REPEAT: usize = 10000;

        let mut original_signal = Vec::with_capacity(self.signal.len() * REPEAT);
        std::mem::swap(&mut self.signal, &mut original_signal);

        for _ in 0..REPEAT {
            self.signal.extend_from_slice(&original_signal);
        }

        let mut to = vec![0; self.signal.len()];
        let offset = self.get_num(0..7);
        for _ in 1..=100 {
            Self::phase_2(offset, &self.signal, &mut to);
            std::mem::swap(&mut self.signal, &mut to);
        }
        Ok(self.get_num(offset..offset + 8).into())
    }
}
