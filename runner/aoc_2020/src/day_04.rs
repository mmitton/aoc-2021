#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
enum Data {
    Byr(String),
    Iyr(String),
    Eyr(String),
    Hgt(String),
    Hcl(String),
    Ecl(String),
    Pid(String),
}

impl Data {
    fn is_valid(&self) -> bool {
        match self {
            Self::Byr(year) => {
                if let Ok(year) = year.parse::<usize>() {
                    (1920..=2002).contains(&year)
                } else {
                    false
                }
            }
            Self::Iyr(year) => {
                if let Ok(year) = year.parse::<usize>() {
                    (2010..=2020).contains(&year)
                } else {
                    false
                }
            }
            Self::Eyr(year) => {
                if let Ok(year) = year.parse::<usize>() {
                    (2020..=2030).contains(&year)
                } else {
                    false
                }
            }
            Self::Hgt(h) => {
                if let Some(h) = h.strip_suffix("in") {
                    if let Ok(h) = h.parse::<usize>() {
                        (59..=76).contains(&h)
                    } else {
                        false
                    }
                } else if let Some(h) = h.strip_suffix("cm") {
                    if let Ok(h) = h.parse::<usize>() {
                        (150..=193).contains(&h)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Self::Hcl(c) => {
                if let Some(c) = c.strip_prefix('#') {
                    if c.len() != 6 {
                        false
                    } else {
                        c.chars()
                            .filter(|c| matches!( c, '0'..='9' | 'a'..='f'))
                            .count()
                            == 6
                    }
                } else {
                    false
                }
            }
            Self::Ecl(c) => matches!(
                c.as_str(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            ),
            Self::Pid(p) => p.len() == 9 && p.chars().filter(|c| c.is_ascii_digit()).count() == 9,
        }
    }
}

struct Passport {
    data: Vec<Data>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        if self.data.len() != 7 {
            return false;
        }

        for d in self.data.iter() {
            if !d.is_valid() {
                return false;
            }
        }

        true
    }
}

pub struct Day04 {
    passports: Vec<Passport>,
}

impl Day04 {
    pub fn new() -> Self {
        Self {
            passports: Vec::new(),
        }
    }
}

impl Runner for Day04 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        let mut passport = Vec::new();
        for line in lines.iter() {
            if line.is_empty() {
                self.passports.push(Passport { data: passport });
                passport = Vec::new();
            }

            for part in line.split_whitespace() {
                let (id, data) = part.split_once(':').unwrap();
                match id {
                    "byr" => passport.push(Data::Byr(data.into())),
                    "iyr" => passport.push(Data::Iyr(data.into())),
                    "eyr" => passport.push(Data::Eyr(data.into())),
                    "hgt" => passport.push(Data::Hgt(data.into())),
                    "hcl" => passport.push(Data::Hcl(data.into())),
                    "ecl" => passport.push(Data::Ecl(data.into())),
                    "pid" => passport.push(Data::Pid(data.into())),
                    "cid" => {}
                    _ => unreachable!(),
                }
            }
        }
        if !passport.is_empty() {
            self.passports.push(Passport { data: passport });
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .passports
            .iter()
            .filter(|p| p.data.len() == 7)
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .passports
            .iter()
            .filter(|p| p.is_valid())
            .count()
            .into())
    }
}
