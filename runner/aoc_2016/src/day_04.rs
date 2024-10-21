#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug)]
struct Room {
    parts: Vec<String>,
    sector_id: usize,
    hash: String,
}

impl FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<&str> = s.split('-').collect();
        let (sector_id, hash) = parts.pop().unwrap().split_once('[').unwrap();
        let hash = hash.strip_suffix(']').unwrap();

        Ok(Self {
            parts: parts.iter().map(|s| (*s).into()).collect(),
            sector_id: sector_id.parse()?,
            hash: hash.into(),
        })
    }
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut counts = [0; 26];
        for part in self.parts.iter() {
            for c in part.chars() {
                counts[(c as u8 - b'a') as usize] += 1;
            }
        }

        let mut counts: Vec<(isize, char)> = counts
            .iter()
            .copied()
            .enumerate()
            .map(|(i, count)| (-(count as isize), (i as u8 + b'a') as char))
            .collect();

        counts.sort();
        let hash_match = self
            .hash
            .chars()
            .zip(counts)
            .filter(|(h, (_, c))| h == c)
            .count();
        hash_match == self.hash.len()
    }

    fn decrypt(&self) -> String {
        let mut decrypted: Vec<String> = Vec::new();

        for part in self.parts.iter() {
            decrypted.push(
                part.chars()
                    .map(|c| {
                        ((((c as u8 - b'a') as usize + self.sector_id) % 26) as u8 + b'a') as char
                    })
                    .collect(),
            );
        }

        decrypted.join(" ")
    }
}

#[derive(Default)]
pub struct Day04 {
    rooms: Vec<Room>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.rooms.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        for room in self.rooms.iter() {
            println!("{room:?}  {}", room.is_valid());
        }
        Ok(self
            .rooms
            .iter()
            .fold(
                0usize,
                |sum, r| if r.is_valid() { sum + r.sector_id } else { sum },
            )
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for room in self.rooms.iter() {
            if room.is_valid() {
                match room.decrypt().as_str() {
                    "northpole object storage" | "very encrypted name" => {
                        return Ok(room.sector_id.into())
                    }
                    _ => {}
                }
            }
        }
        Err(Error::Unsolved)
    }
}
