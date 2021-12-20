#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
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

fn load_input(filename: &str) -> Result<Vec<u8>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let mut lines = BufReader::new(f).lines().map(|l| l.unwrap());

    let mut lengths: Vec<u8> = Vec::new();

    for len in lines.next().unwrap().chars() {
        lengths.push(len as u8);
    }

    Ok(lengths)
}

fn new_key(lengths: &Vec<u8>, idx: usize) -> Vec<u8> {
    let mut key = lengths.clone();

    for c in format!("-{}", idx).chars() {
        key.push(c as u8);
    }

    key.extend_from_slice(&[17, 31, 73, 47, 23]);
    key
}

fn get_hash(key: &Vec<u8>) -> Vec<u8> {
    let mut list = List::new(256);

    for _ in 0..64 {
        for len in key {
            list.process(*len);
        }
    }

    let mut hash = Vec::new();
    for block in (0..255).step_by(16) {
        let mut xor = 0u8;
        for idx in block..block + 16 {
            xor ^= list.numbers[idx];
        }
        hash.push(xor);
    }

    hash
}

fn main() -> Result<(), Error> {
    let input = load_input(INPUT_FILE)?;

    let mut used = 0;
    for i in 0..128 {
        let key = new_key(&input, i);
        let hash = get_hash(&key);

        if cfg!(debug_assertions) {
            print!("{:08b}  ", hash[0]);
        }
        for d in &hash {
            used += d.count_ones();
            if cfg!(debug_assertions) {
                print!("{:02x}", d);
            }
        }
        if cfg!(debug_assertions) {
            println!();
        }
    }

    println!("Used Blocks: {}", used);

    Ok(())
}
