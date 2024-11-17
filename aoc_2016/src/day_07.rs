#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug)]
struct Ip {
    parts: Vec<Part>,
}

impl Ip {
    fn has_tls(&self) -> bool {
        fn is_abba(win: &[char]) -> bool {
            win[0] != win[1] && win[0] == win[3] && win[1] == win[2]
        }
        let mut has_super_abba = false;
        for part in self.parts.iter() {
            match part {
                Part::SuperNet(chars) => {
                    for win in chars.windows(4) {
                        if is_abba(win) {
                            has_super_abba = true;
                        }
                    }
                }
                Part::HyperNet(chars) => {
                    for win in chars.windows(4) {
                        if is_abba(win) {
                            return false;
                        }
                    }
                }
            }
        }
        has_super_abba
    }

    fn has_ssl(&self) -> bool {
        let mut aba = HashSet::default();

        // Find ABA in SuperNets
        for part in self.parts.iter() {
            if let Part::SuperNet(chars) = part {
                for win in chars.windows(3) {
                    if win[0] == win[2] && win[0] != win[1] {
                        aba.insert((win[0], win[1]));
                    }
                }
            }
        }

        // Find BAB in HyperNets
        for part in self.parts.iter() {
            if let Part::HyperNet(chars) = part {
                for win in chars.windows(3) {
                    if win[0] == win[2] && win[0] != win[1] && aba.contains(&(win[1], win[0])) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl FromStr for Ip {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = Vec::new();
        let mut parts = Vec::new();
        let mut is_hyper = false;
        for c in s.chars() {
            match c {
                '[' => {
                    if is_hyper {
                        return Err(Error::InvalidInput(format!("recursive hyper: {s:?}")));
                    } else {
                        if !part.is_empty() {
                            parts.push(Part::SuperNet(part));
                            part = Vec::new();
                        }
                        is_hyper = true;
                    }
                }
                ']' => {
                    if !is_hyper {
                        return Err(Error::InvalidInput(format!(
                            "Not in hypernet, but found ']'.  {s:?}"
                        )));
                    } else {
                        is_hyper = false;
                        parts.push(Part::HyperNet(part));
                        part = Vec::new();
                    }
                }
                _ => part.push(c),
            }
        }

        if !part.is_empty() {
            if is_hyper {
                return Err(Error::InvalidInput(format!("Unterminated hypernet: {s:?}")));
            }
            parts.push(Part::SuperNet(part));
        }

        Ok(Self { parts })
    }
}

#[derive(Debug)]
enum Part {
    SuperNet(Vec<char>),
    HyperNet(Vec<char>),
}

#[derive(Default)]
pub struct Day07 {
    ip_addresses: Vec<Ip>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.ip_addresses.push(line.parse()?);
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

impl Day07 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .ip_addresses
            .iter()
            .filter(|ip| ip.has_tls())
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .ip_addresses
            .iter()
            .filter(|ip| ip.has_ssl())
            .count()
            .into())
    }
}
