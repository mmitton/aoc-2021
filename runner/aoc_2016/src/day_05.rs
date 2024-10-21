#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner, MD5,
};
use std::fmt::Write;

#[derive(Default)]
pub struct Day05 {
    salt: String,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.salt = lines[0].clone();
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut password = String::with_capacity(8);
        let mut hash = self.salt.clone();
        for i in 0.. {
            hash.truncate(self.salt.len());
            write!(hash, "{i}")?;
            let digest = MD5::digest(hash.as_bytes());
            if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0 {
                let v = digest[2] & 0xf;
                password.push(if v < 10 {
                    (v + b'0') as char
                } else {
                    ((v - 10) + b'a') as char
                });
                if password.len() == 8 {
                    break;
                }
            }
        }
        Ok(password.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut password = [None; 8];
        let mut found = 0;
        let mut hash = self.salt.clone();
        for i in 0.. {
            hash.truncate(self.salt.len());
            write!(hash, "{i}")?;
            let digest = MD5::digest(hash.as_bytes());
            if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0 {
                let i = digest[2] & 0xf;
                let v = (digest[3] & 0xf0) >> 4;
                if i < 8 && password[i as usize].is_none() {
                    password[i as usize] = Some(if v < 10 {
                        (v + b'0') as char
                    } else {
                        ((v - 10) + b'a') as char
                    });
                    found += 1;
                    if found == 8 {
                        break;
                    }
                }
            }
        }
        Ok(password
            .iter()
            .map(|c| c.unwrap())
            .collect::<String>()
            .into())
    }
}
