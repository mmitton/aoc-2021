#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

#[derive(Clone)]
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

        return false;
    }

    fn tick(&mut self) {
        let mut pos = 7;
        loop {
            self.chars[pos] += 1;

            match self.chars[pos] as char {
                '{' => {
                    self.chars[pos] = 'a' as u8;
                    if pos == 0 {
                        break;
                    }
                    pos -= 1;
                    continue;
                }
                'i' | 'o' | 'l' => {
                    for i in pos + 1..8 {
                        self.chars[i] = 'a' as u8;
                    }
                    self.chars[pos] += 1;
                }
                _ => {}
            }

            break;
        }
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for d in &self.chars {
            write!(fmt, "{}", *d as char)?;
        }

        Ok(())
    }
}

fn load_input(filename: &str) -> Result<Vec<Password>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut passwords: Vec<Password> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut password = Password { chars: [0; 8] };
        for (i, c) in line.chars().enumerate() {
            match c {
                'i' | 'o' | 'l' => {
                    password.chars[i] = 1 + c as u8;
                    for j in i + 1..8 {
                        password.chars[j] = 'a' as u8;
                    }
                    break;
                }
                _ => password.chars[i] = c as u8,
            }
        }

        passwords.push(password);
    }

    Ok(passwords)
}

fn main() -> Result<(), Error> {
    let mut passwords = load_input(INPUT_FILE)?;

    for password in &mut passwords {
        print!("Password: {:?} {}", password, password.is_valid());
        for _ in 0..2 {
            loop {
                password.tick();
                if password.is_valid() {
                    break;
                }
            }
            print!(" -> {:?}", password);
        }
        println!();
    }

    Ok(())
}
