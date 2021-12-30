#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidOp(String),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

#[derive(Debug)]
enum Op {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl TryFrom<&str> for Op {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.starts_with("rect ") {
            let parts: Vec<&str> = s["rect ".len()..].split("x").collect();
            Ok(Self::Rect(parts[0].parse()?, parts[1].parse()?))
        } else if s.starts_with("rotate row y=") {
            let parts: Vec<&str> = s["rotate row y=".len()..].split(" by ").collect();
            Ok(Self::RotateRow(parts[0].parse()?, parts[1].parse()?))
        } else if s.starts_with("rotate column x=") {
            let parts: Vec<&str> = s["rotate column x=".len()..].split(" by ").collect();
            Ok(Self::RotateCol(parts[0].parse()?, parts[1].parse()?))
        } else {
            Err(Error::InvalidOp(s.to_string()))
        }
    }
}

struct Display {
    w: usize,
    h: usize,
    pixels: Vec<Vec<bool>>,
    buffer: Vec<bool>,
}

impl Display {
    fn new(w: usize, h: usize) -> Self {
        let mut pixels = Vec::new();
        for _ in 0..h {
            pixels.push(vec![false; w]);
        }

        let buffer = if w > h {
            vec![false; w]
        } else {
            vec![false; h]
        };

        Self {
            w: w,
            h: h,
            pixels: pixels,
            buffer: buffer,
        }
    }

    fn process(&mut self, op: &Op) {
        match op {
            Op::Rect(w, h) => {
                for y in 0..*h {
                    for x in 0..*w {
                        self.pixels[y][x] = true;
                    }
                }
            }
            Op::RotateCol(x, offset) => {
                for y in 0..self.h {
                    self.buffer[y] = self.pixels[y][*x];
                }
                for y in 0..self.h {
                    let ny = (y + offset) % self.h;
                    self.pixels[ny][*x] = self.buffer[y];
                }
            }
            Op::RotateRow(y, offset) => {
                for x in 0..self.w {
                    self.buffer[x] = self.pixels[*y][x];
                }
                for x in 0..self.w {
                    let nx = (x + offset) % self.w;
                    self.pixels[*y][nx] = self.buffer[x];
                }
            }
        }
    }

    fn print(&self) {
        println!("Display:");
        for y in 0..self.h {
            for x in 0..self.w {
                print!("{}", if self.pixels[y][x] { "#" } else { "." });
            }
            println!();
        }
        println!();
    }

    fn num_lit(&self) -> usize {
        let mut num_lit = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                if self.pixels[y][x] {
                    num_lit += 1;
                }
            }
        }

        num_lit
    }
}

fn load_input(filename: &str) -> Result<(usize, usize, Vec<Op>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut ops = Vec::new();
    let mut w: usize = 0;
    let mut h: usize = 0;

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        if w == 0 {
            let parts: Vec<&str> = line.split("x").collect();
            w = parts[0].parse()?;
            h = parts[1].parse()?;
        } else {
            ops.push(line.as_str().try_into()?);
        }
    }

    Ok((w, h, ops))
}

fn main() -> Result<(), Error> {
    let (w, h, ops) = load_input(INPUT_FILE)?;

    let mut display = Display::new(w, h);

    for op in &ops {
        display.process(op);
        if cfg!(debug_assertions) {
            display.print();
        }
    }

    println!("Num Lit: {}", display.num_lit());

    Ok(())
}
