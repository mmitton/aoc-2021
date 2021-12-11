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
                        "byr" => passport.birth_year = Some(kv[1].to_string()),
                        "iyr" => passport.issue_year = Some(kv[1].to_string()),
                        "eyr" => passport.expiration_year = Some(kv[1].to_string()),
                        "hgt" => passport.height = Some(kv[1].to_string()),
                        "hcl" => passport.hair_color = Some(kv[1].to_string()),
                        "ecl" => passport.eye_color = Some(kv[1].to_string()),
                        "pid" => passport.passport_id = Some(kv[1].to_string()),
                        "cid" => passport.country_id = Some(kv[1].to_string()),
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
