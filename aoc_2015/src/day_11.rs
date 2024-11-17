#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone, Default)]
struct Password {
    chars: [u8; 8],
}

impl Password {
    fn is_valid(&self) -> bool {
        let mut has_run = false;
        for i in 0..5 {
            if self.chars[i] + 1 == self.chars[i + 1] && self.chars[i] + 2 == self.chars[i + 2] {
                has_run = true;
                break;
            }
        }
        if !has_run {
            return false;
        }

        for i in 0..8 {
            match self.chars[i] as char {
                'i' | 'o' | 'l' => return false,
                _ => {}
            }
        }

        for i in 0..6 {
            if self.chars[i] != self.chars[i + 1] {
                continue;
            }
            for j in i + 2..7 {
                if self.chars[j] == self.chars[j + 1] {
                    return true;
                }
            }
        }

        false
    }

    fn tick(&mut self) {
        let mut pos = 7;
        loop {
            self.chars[pos] += 1;

            match self.chars[pos] as char {
                '{' => {
                    self.chars[pos] = b'a';
                    if pos == 0 {
                        break;
                    }
                    pos -= 1;
                    continue;
                }
                'i' | 'o' | 'l' => {
                    for i in pos + 1..8 {
                        self.chars[i] = b'a';
                    }
                    self.chars[pos] += 1;
                }
                _ => {}
            }

            break;
        }
    }

    fn next(&mut self) {
        self.tick();
        while !self.is_valid() {
            self.tick();
        }
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for d in &self.chars {
            write!(fmt, "{}", *d as char)?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct Day11 {
    password: Password,
}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        for (i, c) in lines[0].chars().enumerate() {
            match c {
                'i' | 'o' | 'l' => {
                    self.password.chars[i] = 1 + c as u8;
                    for j in i + 1..8 {
                        self.password.chars[j] = b'a';
                    }
                    break;
                }
                _ => self.password.chars[i] = c as u8,
            }
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

impl Day11 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.password.next();
        Ok(self.password.to_string().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.password.next();
        self.password.next();
        Ok(self.password.to_string().into())
    }
}
