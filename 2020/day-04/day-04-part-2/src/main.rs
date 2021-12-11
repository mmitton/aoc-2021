#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
}

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<String>,      // byr
    issue_year: Option<String>,      // iyr
    expiration_year: Option<String>, // eyr
    height: Option<String>,          // hgt
    hair_color: Option<String>,      // hcl
    eye_color: Option<String>,       // ecl
    passport_id: Option<String>,     // pid
    country_id: Option<String>,      // cid
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn birth_year(&mut self, v: &str) {
        if v.len() == 4 && v >= "1920" && v <= "2002" {
            self.birth_year = Some(v.to_string());
        }
    }

    fn issue_year(&mut self, v: &str) {
        if v.len() == 4 && v >= "2010" && v <= "2020" {
            self.issue_year = Some(v.to_string());
        }
    }

    fn expiration_year(&mut self, v: &str) {
        if v.len() == 4 && v >= "2020" && v <= "2030" {
            self.expiration_year = Some(v.to_string());
        }
    }

    fn height(&mut self, v: &str) {
        let valid = if v.ends_with("in") {
            match v[..v.len() - 2].parse::<usize>() {
                Ok(h) if h >= 59 && h <= 76 => true,
                _ => false,
            }
        } else if v.ends_with("cm") {
            match v[..v.len() - 2].parse::<usize>() {
                Ok(h) if h >= 150 && h <= 193 => true,
                _ => false,
            }
        } else {
            false
        };

        if valid {
            self.height = Some(v.to_string());
        }
    }

    fn hair_color(&mut self, v: &str) {
        if v.len() == 7 && usize::from_str_radix(&v[1..], 16).is_ok() {
            self.hair_color = Some(v.to_string());
        }
    }

    fn eye_color(&mut self, v: &str) {
        match v {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                self.eye_color = Some(v.to_string());
            }
            _ => {}
        }
    }

    fn passport_id(&mut self, v: &str) {
        if v.len() == 9 && v.trim().parse::<usize>().is_ok() {
            self.passport_id = Some(v.to_string());
        }
    }

    fn country_id(&mut self, v: &str) {
        self.country_id = Some(v.to_string());
    }
}

fn load_passports(filename: &str) -> Result<Vec<Passport>, Error> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport = Some(Passport::default());

    let file = File::open(filename).map_err(|e| Error::IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            if let Some(passport) = passport.take() {
                passports.push(passport);
            }
            passport = Some(Passport::default());
        } else {
            if passport.is_none() {}
            if let Some(passport) = &mut passport {
                let parts = line.split(' ').collect::<Vec<&str>>();
                for part in parts {
                    let kv = part.split(':').collect::<Vec<&str>>();
                    match kv[0] {
                        "byr" => passport.birth_year(kv[1]),
                        "iyr" => passport.issue_year(kv[1]),
                        "eyr" => passport.expiration_year(kv[1]),
                        "hgt" => passport.height(kv[1]),
                        "hcl" => passport.hair_color(kv[1]),
                        "ecl" => passport.eye_color(kv[1]),
                        "pid" => passport.passport_id(kv[1]),
                        "cid" => passport.country_id(kv[1]),
                        _ => return Err(Error::InvalidInput(line.to_string())),
                    }
                }
            }
        }
    }
    if let Some(passport) = passport {
        passports.push(passport);
    }

    Ok(passports)
}

fn main() -> Result<(), Error> {
    let passports = load_passports(INPUT_FILE)?;

    let mut valid = 0usize;
    for passport in &passports {
        if passport.is_valid() {
            valid += 1;
        }
    }

    println!("valid: {}", valid);
    return Ok(());
}
