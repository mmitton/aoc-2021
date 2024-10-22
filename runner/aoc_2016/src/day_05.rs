#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, MD5String, Output, RunOutput, Runner,
    MD5,
};

#[derive(Default)]
pub struct Day05 {
    salt: String,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }
}

struct NumberString {
    bytes: Vec<u8>,
}

impl Default for NumberString {
    fn default() -> Self {
        Self { bytes: vec![b'0'] }
    }
}

impl NumberString {
    fn inc(&mut self) {
        for c in self.bytes.iter_mut().rev() {
            if *c == b'9' {
                *c = b'0';
            } else {
                *c += 1;
                return;
            }
        }
        self.bytes[0] = b'1';
        self.bytes.push(b'0');
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
        let mut hash: MD5String = self.salt.parse()?;
        let mut num = NumberString::default();
        loop {
            num.inc();
            hash.truncate_without_zero(self.salt.len());
            hash.push_bytes(&num.bytes)?;
            let digest = hash.digest(); // MD5::digest(hash.as_bytes());
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
        let mut hash: MD5String = self.salt.parse()?;
        let mut num = NumberString::default();
        loop {
            num.inc();
            hash.truncate_without_zero(self.salt.len());
            hash.push_bytes(&num.bytes)?;
            let digest = hash.digest(); // MD5::digest(hash.as_bytes());
            if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0 {
                let idx = (digest[2] & 0xf) as usize;
                let v = (digest[3] & 0xf0) >> 4;
                if idx < 8 && password[idx].is_none() {
                    password[idx] = Some(if v < 10 {
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
