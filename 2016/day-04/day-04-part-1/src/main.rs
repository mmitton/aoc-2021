#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidInput(String),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

#[derive(Debug, Clone)]
struct Room {
    encrypted: String,
    sector: usize,
    checksum: String,
}

impl TryFrom<&str> for Room {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split("-").collect();

        if parts.len() < 2 {
            return Err(Error::InvalidInput(s.to_string()));
        }

        let encrypted = parts[0..parts.len() - 1].join("-");
        let final_part = parts[parts.len() - 1];
        if !final_part.contains("[") {
            return Err(Error::InvalidInput(s.to_string()));
        }
        let parts: Vec<&str> = final_part.split("[").collect();
        let sector: usize = parts[0].parse()?;
        let checksum = parts[1].replace("]", "").to_string();
        Ok(Room {
            encrypted: encrypted,
            sector: sector,
            checksum: checksum,
        })
    }
}

impl Room {
    fn calc_checksum(&self) -> String {
        let mut letters = [0; 26];
        let mut max = 0usize;
        for letter in self.encrypted.chars() {
            if letter >= 'a' && letter <= 'z' {
                let idx = (letter as u8 - 'a' as u8) as usize;
                letters[idx] += 1;
                if letters[idx] > max {
                    max = letters[idx];
                }
            }
        }

        let mut checksum = String::new();
        for i in (1..=max).rev() {
            for c in 0..26 {
                if letters[c] == i {
                    checksum.push((c as u8 + 'a' as u8) as char);
                }
            }
        }
        if checksum.len() <= 5 {
            checksum
        } else {
            checksum[0..5].to_string()
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<Room>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut rooms: Vec<Room> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        rooms.push(line.as_str().try_into()?);
    }

    Ok(rooms)
}

fn main() -> Result<(), Error> {
    let rooms = load_input(INPUT_FILE)?;
    let mut good_rooms = Vec::new();
    let mut sum = 0usize;

    for room in rooms {
        if room.checksum == room.calc_checksum() {
            sum += room.sector;
            good_rooms.push(room);
        }
    }

    println!("Good Checksums: {}", good_rooms.len());
    println!("Sum Sectors: {}", sum);

    Ok(())
}
