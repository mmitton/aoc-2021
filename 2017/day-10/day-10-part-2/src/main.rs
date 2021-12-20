#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample-2.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

struct List {
    numbers: Vec<u8>,
    pos: usize,
    skip: usize,
}

impl List {
    fn new(size: usize) -> Self {
        let mut numbers: Vec<u8> = Vec::new();

        for i in 0..size {
            numbers.push(i as u8);
        }

        Self {
            numbers: numbers,
            pos: 0,
            skip: 0,
        }
    }

    fn process(&mut self, len: u8) {
        for i in 0..len / 2 {
            let idx1 = (self.pos + i as usize) % self.numbers.len();
            let idx2 = (self.pos + (len - i - 1) as usize) % self.numbers.len();
            let (a, b) = (self.numbers[idx1], self.numbers[idx2]);
            self.numbers[idx2] = a;
            self.numbers[idx1] = b;
        }
        self.pos = (self.pos + len as usize + self.skip) % self.numbers.len();
        self.skip += 1;
    }
}

fn load_input(filename: &str) -> Result<(usize, Vec<u8>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let mut lines = BufReader::new(f).lines().map(|l| l.unwrap());

    let size: usize = lines.next().unwrap().parse().map_err(|e| Error::NAN(e))?;
    let mut lengths: Vec<u8> = Vec::new();

    for len in lines.next().unwrap().chars() {
        lengths.push(len as u8);
    }
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    Ok((size, lengths))
}

fn main() -> Result<(), Error> {
    let (size, lengths) = load_input(INPUT_FILE)?;

    println!("size: {}  lengths: {:?}", size, lengths);

    let mut list = List::new(size);

    for _ in 0..64 {
        for len in &lengths {
            list.process(*len);
            if cfg!(debug_assertions) {
                println!("List: pos:{}  numbers:{:?}", list.pos, list.numbers);
            }
        }
    }

    print!("Hash: ");
    for block in (0..255).step_by(16) {
        let mut xor = 0u8;
        for idx in block..block + 16 {
            xor ^= list.numbers[idx];
        }
        print!("{:02x}", xor);
    }
    println!();

    Ok(())
}
