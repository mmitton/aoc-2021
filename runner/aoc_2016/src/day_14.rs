#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, MD5String, Output, RunOutput, Runner,
};
use std::fmt::Write;

#[derive(Debug)]
struct Hash {
    threes: Vec<u8>,
    fives: Vec<u8>,
}

impl Hash {
    fn new(digits: &[u8]) -> Self {
        let mut hash = Hash {
            threes: Vec::new(),
            fives: Vec::new(),
        };

        let mut i = 0;
        while i < digits.len() - 2 {
            if digits[i] == digits[i + 1] && digits[i] == digits[i + 2] {
                if hash.threes.is_empty() {
                    // !hash.threes.contains(&digits[i]) {
                    hash.threes.push(digits[i]);
                }
                if i < digits.len() - 4 && digits[i] == digits[i + 3] && digits[i] == digits[i + 4]
                {
                    if !hash.fives.contains(&digits[i]) {
                        hash.fives.push(digits[i]);
                    }
                    i += 2;
                }

                i += 3;
            } else {
                i += 1;
            }
        }

        hash
    }
}

#[derive(Default)]
pub struct Day14 {
    hashes: Vec<Hash>,
    md5: MD5String,
    salt_len: usize,
}

impl Day14 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_hash(&mut self, n: usize, part1: bool) -> Result<&Hash, Error> {
        let mut hash: [u8; 32] = [0; 32];

        while self.hashes.len() <= n {
            self.md5.truncate(self.salt_len);
            write!(&mut self.md5, "{}", self.hashes.len())?;

            let result = if part1 {
                self.md5.digest()
            } else {
                let mut md5 = self.md5;
                let mut result: [u8; 16] = [0; 16];
                for _ in 0..=2016 {
                    result = md5.digest();
                    md5.truncate_without_zero(0);
                    for (i, v) in result.iter().enumerate() {
                        macro_rules! set {
                            ($i:expr, $v:expr) => {{
                                if $v <= 9 {
                                    hash[$i] = $v + b'0';
                                } else {
                                    hash[$i] = $v - 10 + b'a';
                                }
                            }};
                        }

                        set!(i * 2, v >> 4);
                        set!(i * 2 + 1, v & 0xf);
                    }
                    md5.push_bytes(&hash)?;
                }
                result
            };

            for (i, r) in result.iter().enumerate() {
                hash[i * 2] = r >> 4;
                hash[i * 2 + 1] = r & 0xf;
            }
            self.hashes.push(Hash::new(&hash));
        }

        Ok(&self.hashes[n])
    }

    fn solve(&mut self, part1: bool) -> Result<usize, Error> {
        let mut keys = Vec::new();

        'search_loop: for i in 0..usize::MAX {
            let threes = self.get_hash(i, part1)?.threes.clone();

            if !threes.is_empty() {
                for n in &threes {
                    for j in i + 1..=i + 1000 {
                        let hash = self.get_hash(j, part1)?;
                        if hash.fives.contains(n) {
                            keys.push(i);
                            if keys.len() == 64 {
                                return Ok(i);
                            }
                            continue 'search_loop;
                        }
                    }
                }
            }
        }
        Err(Error::Unsolved)
    }
}

impl Runner for Day14 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.salt_len = lines[0].len();
        write!(&mut self.md5, "{}", lines[0])?;
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.solve(true)?.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.solve(false)?.into())
    }
}
